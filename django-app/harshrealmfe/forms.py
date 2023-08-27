from django import forms
from django.forms.models import inlineformset_factory
from .models import AlienRaceName

class AlienRaceNameForm(forms.ModelForm):
    class Meta:
        model = AlienRaceName
        fields =[
            "dice_roll"
            ,"race_name"
        ]
        widgets = {
            "dice_roll": forms.IntegerField(attrs={"class": "form-control"}),
            "race_name": forms.TextInput(attrs={"class": "form_control"})
        }

# AlienRacenameFormSet = inlineformset_factory(
#     AlienRaceName,
#     form=AlienRaceNameForm,
#     can_delete=True
# )