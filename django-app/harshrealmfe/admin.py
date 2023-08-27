from django.contrib import admin
from import_export import resources

from .models import AlienRaceName, MercsForHire, StarshipProblem, Rumor, StarshipName, RandomItem, OddSituation, \
    ReasonCharactersAreTogether, MissionHook, ChatSession, ChatMessage

# Register your models here.
admin.site.register(AlienRaceName)
admin.site.register(MercsForHire)
admin.site.register(StarshipProblem)
admin.site.register(Rumor)
admin.site.register(StarshipName)
admin.site.register(RandomItem)
admin.site.register(OddSituation)
admin.site.register(ReasonCharactersAreTogether)
admin.site.register(MissionHook)
admin.site.register(ChatSession)
admin.site.register(ChatMessage)

class AlienRaceNameResource(resources.ModelResource):
    class Meta:
        model = AlienRaceName
