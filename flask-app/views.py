from flask_admin.contrib.sqla import ModelView


class SciFiCorpNameGenView(ModelView):
    column_searchable_list = ['dice_roll', 'prefix', 'suffix']
    column_editable_list = ['dice_roll', 'prefix', 'suffix']
    # create_modal = True


class CyberneticsImplantNamesView(ModelView):
    column_searchable_list = ['dice_roll', 'descriptor', 'thing']
    column_editable_list = ['dice_roll', 'descriptor', 'thing']

class DerelictStarshipView(ModelView):
    column_searchable_list = ['dice_roll', 'name']
    column_editable_list = ['dice_roll', 'name', 'notes']
    column_list = ['dice_roll', 'name', 'notes']