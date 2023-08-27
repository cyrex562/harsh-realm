from django.db import models


# Create your models here.
class AlienRaceName(models.Model):
    dice_roll = models.IntegerField()
    race_name = models.CharField(max_length=200)

    def __str__(self):
        return f"dice roll: {self.dice_roll}, race name: {self.race_name}"


class MercsForHire(models.Model):
    name = models.CharField(max_length=200)
    description = models.TextField()

    def __str__(self):
        return f"name: {self.name}, description: {self.description}"


class StarshipProblem(models.Model):
    dice_roll = models.IntegerField()
    description = models.TextField()


class Rumor(models.Model):
    dice_roll = models.IntegerField()
    description = models.TextField()


class StarshipName(models.Model):
    dice_roll = models.IntegerField()
    name = models.CharField(max_length=200)

class RandomItem(models.Model):
    dice_roll = models.IntegerField()
    place = models.CharField(max_length=200)
    item = models.TextField()

class OddSituation(models.Model):
    dice_roll = models.IntegerField()
    location = models.CharField(max_length=200)
    situation = models.TextField()

class ReasonCharactersAreTogether(models.Model):
    dice_roll = models.IntegerField()
    reason = models.TextField()


class MissionHook(models.Model):
    dice_roll = models.IntegerField()
    hook = models.TextField()


class ChatSession(models.Model):
    name = models.CharField(max_length=200)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    def __str__(self):
        return f"name={self.name} created={self.created_at} updated={self.updated_at}"


class ChatMessage(models.Model):
    message = models.TextField()
    chat_session = models.ForeignKey(ChatSession, on_delete=models.CASCADE)
    created_at = models.DateTimeField(auto_now_add=True)


class BannedCargo(models.Model):
    dice_roll = models.IntegerField()
    cargo = models.TextField()

