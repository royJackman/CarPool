from django.db import models

# Create your models here.
class Car(models.Model):
    name = models.CharField(max_length=128)

class User(models.Model):
    name = models.CharField(max_length=128)

class Reservation(models.Model):
    description = models.CharField(max_length=256)
    start_time = models.DateTimeField("start time")
    end_time = models.DateTimeField("end time")
    car_id = models.ForeignKey(Car, on_delete=models.CASCADE)
    user_id = models.ForeignKey(User, on_delete=models.CASCADE)
