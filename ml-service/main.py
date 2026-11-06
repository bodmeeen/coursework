from fastapi import FastAPI
from pydantic import BaseModel
import pandas as pd
import nltk
from nltk.sentiment import SentimentIntensityAnalyzer
from transformers import pipeline
import torch

# Словник тональності
nltk.download('vader_lexicon', quiet=True)
sia = SentimentIntensityAnalyzer()

# Перевірка чи доступна відеокарта
device = 0 if torch.cuda.is_available() else -1

# Завантаження Zero-Shot модель BART
classifier = pipeline("zero-shot-classification", model="facebook/bart-large-mnli", device=device)

app = FastAPI()

class AnalysisRequest(BaseModel):
    data: list[dict]

CATEGORY_MAPPING = {
    "shipping and delivery": "Доставка та логістика",
    "product quality": "Якість товару",
    "price and value": "Ціна та вартість",
    "customer service": "Обслуговування клієнтів",
    "other": "Інше"
}
CANDIDATE_LABELS = list(CATEGORY_MAPPING.keys())

@app.post("/analyze")
def analyze_feedback(request: AnalysisRequest):
    df = pd.DataFrame(request.data)
    
    # Пошук текстової колонки
    text_col = 'text' if 'text' in df.columns else df.columns[0]
    df[text_col] = df[text_col].fillna('')

    # Визначення тональності 
    def get_sentiment(text):
        score = sia.polarity_scores(str(text))['compound']
        if score > 0.05: return 'Позитивний'
        if score < -0.05: return 'Негативний'
        return 'Нейтральний'

    df['sentiment'] = df[text_col].apply(get_sentiment)

    # Сегментація (Класифікація)
    def get_category(text):
        if not str(text).strip():
            return CATEGORY_MAPPING["other"]
        
        # Аналізування тексту та повернення категорій відсортованих за ймовірністю
        result = classifier(str(text), candidate_labels=CANDIDATE_LABELS)
        
        # Діставання категорії з найбільшим відсотком збігу 
        top_label = result['labels'][0]
        return CATEGORY_MAPPING.get(top_label, top_label)

    # Залишити назву колонки "customer_segment" для правильного відображення на фронтенді
    df['customer_segment'] = df[text_col].apply(get_category)

    return {
        "status": "success",
        "processed_rows": len(df),
        "data": df.to_dict(orient="records")
    }

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)