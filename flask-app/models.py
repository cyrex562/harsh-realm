from database import db


class SciFiCriminal(db.Model):
    __tablename__ = 'scifi_criminals'
    id = db.Column(db.Integer, primary_key=True)
    dice_roll = db.Column(db.Integer, nullable=False)
    name = db.Column(db.String)
    nickname = db.Column(db.String)
    occupation = db.Column(db.String)
    crime = db.Column(db.String)
    quote = db.Column(db.String)


class SciFiJunk(db.Model):
    __tablename__ = 'scifi_junk'
    id = db.Column(db.Integer, primary_key=True)
    dice_roll = db.Column(db.Integer, nullable=False)
    item = db.Column(db.String)
    
    
class SciFiNPCs(db.Model):
    __tablename__ = 'scifi_npcs'
    id = db.Column(db.Integer, primary_key=True)
    dice_roll = db.Column(db.Integer, nullable=False)
    name = db.Column(db.String)
    occupation = db.Column(db.String)
    secret = db.Column(db.String)
    need = db.Column(db.String)
    demeanor = db.Column(db.String)
    
    
class CyberpunkOddSituation(db.Model):
    __tablename__ = 'cyberpunk_odd_situations'
    id = db.Column(db.Integer, primary_key=True)
    dice_roll = db.Column(db.Integer, nullable=False, server_default='0')
    location = db.Column(db.String)
    situation = db.Column(db.Text)

class SciFiCorpNameGen(db.Model):
    __tablename__ = 'sci_fi_corp_name_gen'
    id = db.Column(db.Integer, primary_key=True)
    dice_roll = db.Column(db.Integer, nullable=False, server_default='0')
    prefix = db.Column(db.String)
    suffix = db.Column(db.String)
    

class CyberneticsImplantNames(db.Model):
    __tablename__  = "cybernetics_implant_names"
    id = db.Column(db.Integer, primary_key = True)
    dice_roll = db.Column(db.Integer, nullable=False, server_default='0')
    descriptor = db.Column(db.String)
    thing = db.Column(db.String)

class DerelictStarship(db.Model):
    __tablename__ = "derelict_starships"
    id = db.Column(db.Integer, primary_key=True)
    dice_roll = db.Column(db.Integer, nullable=False, server_default='')
    name = db.Column(db.String)
    notes = db.relationship('DerelictStarshipNote', backref='starship')


class DerelictStarshipNote(db.Model):
    __tablename__ = "derelict_starship_notes"
    id = db.Column(db.Integer, primary_key=True)
    starship_id = db.Column(db.Integer, db.ForeignKey(DerelictStarship.id))
    note = db.Column(db.Text)