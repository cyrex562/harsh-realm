from .chat_message_ops import process_chat_msg
from django.shortcuts import render
from django.http import HttpResponse
from django.template import loader
from django.views import generic
from django.urls import reverse
from django.views.decorators.http import require_http_methods

from .models import AlienRaceName, MercsForHire, StarshipProblem, Rumor, StarshipName, RandomItem, OddSituation, \
    ReasonCharactersAreTogether, MissionHook, ChatMessage, ChatSession, BannedCargo

import logging

logger = logging.getLogger(__name__)

# Create your views here.
def index(request):
    template = loader.get_template("index.html")
    context = {}
    return render(request, "index.html", context)


def list_alien_race_names(request):
    alien_race_names = AlienRaceName.objects.all()
    return render(request, 'list_alien_race_names.html', {'alien_race_names': alien_race_names})


@require_http_methods(['DELETE'])
def delete_alien_race_name(request, id):
    AlienRaceName.objects.filter(id=id).delete()
    alien_race_names = AlienRaceName.objects.all()
    return render(request, 'alien_race_names_list.html', {'alien_race_names': alien_race_names})


@require_http_methods(['POST'])
def update_alien_race_name(request, id):
    dice_roll = request.POST.get('dice_roll')
    race_name = request.POST.get('race_name')
    AlienRaceName.objects.filter(id=id).update(dice_roll=dice_roll, race_name=race_name)
    alien_race_names = AlienRaceName.objects.all()
    return render(request, 'alien_race_names_list.html', {'alien_race_names': alien_race_names})


@require_http_methods(['POST'])
def create_alien_race_name(request):
    dice_roll = request.POST.get('dice_roll')
    race_name = request.POST.get('race_name')
    # alien_race_name = AlienRaceName(dice_roll=dice_roll,race_name=race_name)
    AlienRaceName.objects.create(dice_roll=dice_roll, race_name=race_name)
    alien_race_names = AlienRaceName.objects.all()
    return render(request, 'alien_race_names_list.html', {'alien_race_names': alien_race_names})


def list_mercs_for_hire(request):
    mercs_for_hire = MercsForHire.objects.all()
    return render(request, 'list_mercs_for_hire.html',
                  {'mercs_for_hire': mercs_for_hire})


@require_http_methods(['DELETE'])
def delete_merc_for_hire(request, id):
    MercsForHire.objects.filter(id=id).delete()
    mercs_for_hire = MercsForHire.objects.all()
    return render(request,
                  'mercs_for_hire_list.html',
                  {'mercs_for_hire': mercs_for_hire})


@require_http_methods(['POST'])
def update_merc_for_hire(request, id):
    name = request.POST.get('name')
    description = request.POST.get('description')
    MercsForHire.objects.filter(id=id).update(name=name, description=description)
    mercs_for_hire = MercsForHire.objects.all()
    return render(request,
                  'mercs_for_hire_list.html',
                  {'mercs_for_hire': mercs_for_hire})


@require_http_methods(['POST'])
def create_merc_for_hire(request):
    name = request.POST.get('name')
    description = request.POST.get('description')
    MercsForHire.objects.create(name=name, description=description)
    mercs_for_hire = MercsForHire.objects.all()
    return render(request,
                  'mercs_for_hire_list.html',
                  {'mercs_for_hire': mercs_for_hire})


def list_starship_problems(request):
    starship_problems = StarshipProblem.objects.all()
    return render(request,
                  "list_starship_problems.html",
                  {"starship_problems": starship_problems})


@require_http_methods(['DELETE'])
def delete_starship_problem(request, id):
    StarshipProblem.objects.filter(id=id).delete()
    starship_problems = StarshipProblem.objects.all()
    return render(request,
                  "starship_problems_list.html",
                  {"starship_problems": starship_problems})


@require_http_methods(['POST'])
def update_starship_problem(request, id):
    dice_roll = request.POST.get('dice_roll')
    description = request.POST.get('description')
    StarshipProblem.objects.filter(id=id).update(dice_roll=dice_roll, description=description)
    starship_problems = StarshipProblem.objects.all()
    return render(request,
                  "starship_problems_list.html",
                  {"starship_problems": starship_problems})


@require_http_methods(['POST'])
def create_starship_problem(request):
    dice_roll = request.POST.get('dice_roll')
    description = request.POST.get('description')
    StarshipProblem.objects.create(dice_roll=dice_roll, description=description)
    starship_problems = StarshipProblem.objects.all()
    return render(request,
                  "starship_problems_list.html",
                  {"starship_problems": starship_problems})


def list_rumors(request):
    rumors = Rumor.objects.all()
    return render(request, "list_rumors.html", {"rumors": rumors})


@require_http_methods(['DELETE'])
def delete_rumor(request, id):
    Rumor.objects.filter(id=id).delete()
    rumors = Rumor.objects.all()
    return render(request, "rumors_list.html", {"rumors": rumors})


@require_http_methods(['POST'])
def update_rumor(request, id):
    dice_roll = request.POST.get('dice_roll')
    description = request.POST.get('description')
    Rumor.objects.filter(id=id).update(dice_roll=dice_roll, description=description)
    rumors = Rumor.objects.all()
    return render(request, "rumors_list.html", {"rumors": rumors})


@require_http_methods(['POST'])
def create_rumor(request):
    dice_roll = request.POST.get('dice_roll')
    description = request.POST.get('description')
    Rumor.objects.create(dice_roll=dice_roll, description=description)
    rumors = Rumor.objects.all()
    return render(request, "rumors_list.html", {"rumors": rumors})


def list_starship_names(request):
    starship_names = StarshipName.objects.all()
    return render(request, "list_starship_names.html", {"starship_names": starship_names})


@require_http_methods(['DELETE'])
def delete_starship_name(request, id):
    StarshipName.objects.filter(id=id).delete()
    starship_names = StarshipName.objects.all()
    return render(request, "starship_names_list.html", {"starship_names": starship_names})


@require_http_methods(['POST'])
def update_starship_name(request, id):
    dice_roll = request.POST.get('dice_roll')
    name = request.POST.get('name')

    StarshipName.objects.filter(id=id).update(dice_roll=dice_roll, name=name)
    starship_names = StarshipName.objects.all()
    return render(request, "starship_names_list.html", {"starship_names": starship_names})


@require_http_methods(['POST'])
def create_starship_name(request):
    dice_roll = request.POST.get('dice_roll')
    name = request.POST.get('name')
    StarshipName.objects.create(dice_roll=dice_roll, name=name)
    starship_names = StarshipName.objects.all()
    return render(request, "starship_names_list.html", {"starship_names": starship_names})


def list_random_items(request):
    random_items = RandomItem.objects.all().order_by('-id')
    return render(request, "list_random_items.html", {"random_items": random_items})


@require_http_methods(['DELETE'])
def delete_random_item(request, id):
    RandomItem.objects.filter(id=id).delete()
    random_items = RandomItem.objects.all().order_by('-id')
    return render(request, "random_items_list.html", {"random_items": random_items})


@require_http_methods(['POST'])
def update_random_item(request, id):
    dice_roll = request.POST.get('dice_roll')
    item = request.POST.get('item')
    place = request.POST.get('place')
    RandomItem.objects.filter(id=id).update(dice_roll=dice_roll, item=item, place=place)
    random_items = RandomItem.objects.all().order_by('-id')
    return render(request, "random_items_list.html", {"random_items": random_items})


@require_http_methods(['POST'])
def create_random_item(request):
    dice_roll = request.POST.get('dice_roll')
    item = request.POST.get('item')
    place = request.POST.get('place')
    RandomItem.objects.create(dice_roll=dice_roll, item=item, place=place)
    random_items = RandomItem.objects.all().order_by('-id')
    return render(request, "random_items_list.html", {"random_items": random_items})


def list_odd_situations(request):
    odd_situations = OddSituation.objects.all()
    return render(request, "list_odd_situations.html", {"odd_situations": odd_situations})


@require_http_methods(['DELETE'])
def delete_odd_situation(request, id):
    OddSituation.objects.filter(id=id).delete()
    odd_situations = OddSituation.objects.all()
    return render(request, "odd_situations_list.html", {"odd_situations": odd_situations})


@require_http_methods(['POST'])
def update_odd_situation(request, id):
    dice_roll = request.POST.get('dice_roll')
    location = request.POST.get('location')
    situation = request.POST.get('situation')
    OddSituation.objects.filter(id=id).update(dice_roll=dice_roll, situation=situation, location=location)
    odd_situations = OddSituation.objects.all()
    return render(request, "odd_situations_list.html", {"odd_situations": odd_situations})


@require_http_methods(['POST'])
def create_odd_situation(request):
    dice_roll = request.POST.get('dice_roll')
    location = request.POST.get('location')
    situation = request.POST.get('situation')
    OddSituation.objects.create(dice_roll=dice_roll, situation=situation, location=location)
    odd_situations = OddSituation.objects.all()
    return render(request, "odd_situations_list.html", {"odd_situations": odd_situations})


def list_reasons_characters_are_together(request):
    reasons_characters_are_together = ReasonCharactersAreTogether.objects.all()
    return render(request, "list_reasons_characters_are_together.html",
                  {"reasons_characters_are_together": reasons_characters_are_together})


@require_http_methods(['DELETE'])
def delete_reason_characters_are_together(request, id):
    ReasonCharactersAreTogether.objects.filter(id=id).delete()
    reasons_characters_are_together = ReasonCharactersAreTogether.objects.all()
    return render(request, "reasons_characters_are_together_list.html",
                  {"reasons_characters_are_together": reasons_characters_are_together})


@require_http_methods(['POST'])
def update_reason_characters_are_together(request, id):
    dice_roll = request.POST.get('dice_roll')
    reason = request.POST.get('reason')
    ReasonCharactersAreTogether.objects.filter(id=id).update(dice_roll=dice_roll, reason=reason)
    reasons_characters_are_together = ReasonCharactersAreTogether.objects.all()
    return render(request, "reasons_characters_are_together_list.html",
                  {"reasons_characters_are_together": reasons_characters_are_together})


@require_http_methods(['POST'])
def create_reason_characters_are_together(request):
    dice_roll = request.POST.get('dice_roll')
    reason = request.POST.get('reason')
    ReasonCharactersAreTogether.objects.create(dice_roll=dice_roll, reason=reason)
    reasons_characters_are_together = ReasonCharactersAreTogether.objects.all()
    return render(request, "reasons_characters_are_together_list.html",
                  {"reasons_characters_are_together": reasons_characters_are_together})


def list_mission_hooks(request):
    mission_hooks = MissionHook.objects.all()
    return render(request, "list_mission_hooks.html", {"mission_hooks": mission_hooks})


@require_http_methods(['DELETE'])
def delete_mission_hook(request, id):
    MissionHook.objects.filter(id=id).delete()
    mission_hooks = MissionHook.objects.all()
    return render(request, "mission_hook_list.html", {"mission_hooks": mission_hooks})


@require_http_methods(['POST'])
def update_mission_hook(request, id):
    dice_roll = request.POST.get('dice_roll')
    hook = request.POST.get('hook')
    MissionHook.objects.filter(id=id).update(dice_roll=dice_roll, hook=hook)
    mission_hooks = MissionHook.objects.all()
    return render(request, "mission_hook_list.html", {"mission_hooks": mission_hooks})


@require_http_methods(['POST'])
def create_mission_hook(request):
    dice_roll = request.POST.get('dice_roll')
    hook = request.POST.get('hook')
    MissionHook.objects.create(dice_roll=dice_roll, hook=hook)
    mission_hooks = MissionHook.objects.all()
    return render(request, "mission_hook_list.html", {"mission_hooks": mission_hooks})


def client_window(request):
    logger.info("client_window")
    chat_messages = ChatMessage.objects.none()
    chat_sessions = ChatSession.objects.all()
    sel_chat_session = None
    loaded_chat_session = None
    return render(request, "client.html", {"chat_messages": chat_messages, "chat_sessions": chat_sessions, "sel_chat_session": sel_chat_session})


@require_http_methods(['POST'])
def receive_chat_msg(request):
    logger.info("receive_chat_msg")
    msg = request.POST.get('msg')
    print(f"message: {msg}")



    chat_sessions = ChatSession.objects.all()
    loaded_chat_session_id = request.POST.get('loaded_chat_session_id')
    sel_chat_session = ChatSession.objects.get(id=loaded_chat_session_id)
    loaded_chat_session = sel_chat_session
    ChatMessage.objects.create(message=msg, chat_session=ChatSession.objects.get(id=loaded_chat_session.id), chat_session_id=loaded_chat_session.id)

    response_msg = process_chat_msg(msg)
    ChatMessage.objects.create(message=response_msg, chat_session=loaded_chat_session, chat_session_id=loaded_chat_session.id)

    chat_messages = ChatMessage.objects.filter(chat_session_id__exact=loaded_chat_session.id)
    return render(request, "chat_window.html", {"chat_messages": chat_messages, "chat_sessions": chat_sessions, "sel_chat_session": sel_chat_session, "loaded_chat_session": loaded_chat_session})


@require_http_methods(['POST'])
def create_session(request):
    logger.info("create_session")
    session_name = request.POST.get('session_name')
    sel_chat_session = ChatSession.objects.create(name=session_name)
    loaded_chat_session = sel_chat_session
    sessions = ChatSession.objects.all()
    chat_messages = ChatMessage.objects.filter(chat_session_id__exact=loaded_chat_session.id)
    return render(request, "chat_window.html", {"chat_sessions": sessions, "chat_messages:": chat_messages, "sel_chat_session": sel_chat_session, "loaded_chat_session": loaded_chat_session})


@require_http_methods(["POST"])
def load_session(request):
    logger.info("load_session")
    chat_session_id = request.POST.get('chat_sessions')
    sessions = ChatSession.objects.all()
    sel_chat_session = ChatSession.objects.filter(id=chat_session_id).first()
    loaded_chat_session = sel_chat_session
    chat_messages = ChatMessage.objects.filter(chat_session_id__exact=loaded_chat_session.id).all()
    return render(request, "chat_window.html", {"chat_sessions": sessions, "chat_messages": chat_messages, "sel_chat_session": sel_chat_session, "loaded_chat_session": loaded_chat_session})


def list_banned_cargo(request):
    banned_cargoes = BannedCargo.objects.all()
    return render(request, "list_banned_cargo.html", {"banned_cargoes": banned_cargoes})

@require_http_methods(['DELETE'])
def delete_banned_cargo(request, id):
    BannedCargo.objects.filter(id=id).delete()
    banned_cargoes = BannedCargo.objects.all()
    return render(request, "banned_cargo_list.html", {"banned_cargoes": banned_cargoes})

@require_http_methods(['POST'])
def update_banned_cargo(request, id):
    dice_roll = request.POST.get('dice_roll')
    cargo = request.POST.get('cargo')
    BannedCargo.objects.filter(id=id).update(dice_roll=dice_roll, cargo=cargo)
    banned_cargoes = BannedCargo.objects.all()
    return render(request, "banned_cargo_list.html", {"banned_cargoes": banned_cargoes})

@require_http_methods(['POST'])
def create_banned_cargo(request):
    dice_roll = request.POST.get('dice_roll')
    cargo = request.POST.get('cargo')
    BannedCargo.objects.create(dice_roll=dice_roll, cargo=cargo)
    banned_cargoes = BannedCargo.objects.all()
    return render(request, "banned_cargo_list.html", {"banned_cargoes": banned_cargoes})