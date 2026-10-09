from django.http import JsonResponse
from django.urls import path

def hello(request):
    return JsonResponse({"message": "Hello from Codedock Python Django Example!"})

urlpatterns = [
    path("", hello),
]