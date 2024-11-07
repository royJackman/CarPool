from django.shortcuts import render
from django.http import HttpResponse, HttpRequest
from .models import Reservation

# Create your views here.
def index(request: HttpRequest) -> HttpResponse:
    # Reservation.objects.filter(start_time__date_joined__gt=now())
    return HttpResponse("This is the reservations page")
