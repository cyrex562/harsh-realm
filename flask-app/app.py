# app.py
import os

from flask import Flask, render_template, request, send_from_directory
from flask_assets import Bundle, Environment
from flask_admin import Admin
from flask_sqlalchemy import SQLAlchemy
from flask_admin.contrib.sqla import ModelView
from flask_migrate import Migrate

import dice_roller
import models
import mythic_fate_chart

from database import db
from views import CyberneticsImplantNamesView, DerelictStarshipView, SciFiCorpNameGenView

app = Flask(__name__)
app.config.from_pyfile('config.py')
db.init_app(app)
# alembic = Alembic()
# alembic.init_app(app)
migrate = Migrate(app, db, render_as_batch=True)

# babel = Babel(app)

admin = Admin(app, name="Harsh Realm")
assets = Environment(app)
css = Bundle("src/main.css", output="dist/main.css")
js = Bundle("src/*.js", output="dist/main.js")

assets.register("css", css)
assets.register("js", js)
css.build()
js.build()

mythic_fate_chart.init_fate_chart()

admin.add_view(ModelView(models.SciFiCriminal, db.session))
admin.add_view(ModelView(models.SciFiJunk, db.session))
admin.add_view(ModelView(models.SciFiNPCs, db.session))
admin.add_view(ModelView(models.CyberpunkOddSituation, db.session))
admin.add_view(SciFiCorpNameGenView(models.SciFiCorpNameGen, db.session))
admin.add_view(CyberneticsImplantNamesView(models.CyberneticsImplantNames, db.session))
admin.add_view(DerelictStarshipView(models.DerelictStarship, db.session))


@app.route("/favicon.ico")
def favicon():
    return send_from_directory(os.path.join(app.root_path, 'static/src/favicon'), 'favicon.ico',
                               mimetype='image/vnd.microsoft.com')


@app.route("/")
def home():
    return render_template("home.html")


@app.route("/roll_fate_chart", methods=["GET", "POST"])
def roll_fate_chart():
    odds_values = mythic_fate_chart.get_odd_vals()
    if request.method == "POST":
        odds = request.form.get("odds")
        chaos_factor = int(request.form.get("chaos_factor"))
        rolls = dice_roller.roll_dice(100, 1)
        roll = rolls[0]
        fate_result = mythic_fate_chart.consult_fate_chart(odds, roll, chaos_factor)
        xyes, value, xno = mythic_fate_chart.get_fate_chart_cell(odds, chaos_factor)
        return render_template("roll_fate_chart.html", odds_values=odds_values, selected_val=odds,
                               chaos_factor=chaos_factor, dice_roll_val=roll,
                               fate_chart_result=fate_result, exc_yes_range=xyes, value_range=value, exc_no_range=xno)
    else:
        return render_template("roll_fate_chart.html", odds_values=odds_values, selected_val="", chaos_factor=1,
                               dice_roll_val=0, fate_chart_result="-", exc_yes_range="-", value_range="-",
                               exc_no_range="-")


@app.route('/roll_dice', methods=["GET", "POST"])
def roll_dice():
    if request.method == "POST":
        num_dice = int(request.form.get("num_dice"))
        num_sides = int(request.form.get("num_sides"))
        rolls = dice_roller.roll_dice(num_sides, num_dice)
        out_string = " ".join(str(x) for x in rolls)
        return render_template("roll_dice.html", rolls=out_string, num_dice=num_dice, num_sides=num_sides)
    else:
        num_dice = 0
        num_sides = 0
        return render_template("roll_dice.html", rolls="", num_dice=num_dice, num_sides=num_sides)

with app.app_context():
    db.create_all()

if __name__ == "__main__":
    app.run(debug=True)
