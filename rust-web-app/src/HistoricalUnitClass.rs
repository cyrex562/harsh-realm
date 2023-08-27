// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.HistoricalUnitClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.IO;
// usingSystem.Runtime.Serialization;
// usingSystem.Runtime.Serialization.Formatters.Binary;

namespace WindowsApplication1
{
  [Serializable]
  pub class HistoricalUnitClass : ISerializable
  {
    pub Name: String;
    pub Type: i32;
    pub ID: i32;
    pub Counter: i32;
    pub CounterString: String;
    pub TempDefend: i32;
    pub TempAttack: i32;
    pub TempStance: i32;
    pub TempTargetX: i32;
    pub TempTargetY: i32;
    pub TempTargetAttackX: i32;
    pub TempTargetAttackY: i32;
    pub TempRegime: i32;
    pub NoSplit: bool;
    pub UseRomans: bool;
    pub MaxPresent: i32;
    pub StartSize: i32;
    pub SmallGfx: i32;
    pub CommanderName: String;
    pub CommanderFileName: String;
    pub CommanderSpriteID: i32;
    pub OverdrawFileName: String;
    pub OverdrawSpriteID: i32;
    pub DeckCardCounter: i32;
    pub DeckCard: Vec<i32>;
    pub DeckChance: Vec<i32>;
    pub AutoEventCounter: i32;
    pub AutoEvent: Vec<i32>;
    pub AutoChance: Vec<i32>;
    pub HandCardCounter: i32;
    pub HandCard: Vec<i32>;
    pub PP: i32;
    pub Fixed: bool;
    pub NameCounter: i32;
    pub NameCounterBackup: i32;
    pub PercentOldName: i32;
    pub CombatMod: i32;
    pub MoraleMod: i32;
    pub StaffSize: i32;
    pub Model: bool;
    pub Pool: bool;
    pub Red: i32;
    pub Green: i32;
    pub Blue: i32;
    pub SubParts: Vec<i32>;
    pub Designation: Vec<i32>;
    pub DesignationSmall: Vec<i32>;
    pub BattleGroup: i32;
    pub AIlist: i32;
    pub ModelMaster: i32;
    pub Descript: String;
    pub Xp: i32;
    pub TempVar1: i32;
    pub TempVar2: i32;
    pub TempVar3: i32;
    pub TempVar4: i32;
    pub TempVar5: i32;
    pub People: i32;
    pub UsePeopleGfx: i32;
    pub Level1: i32;
    pub Level2: i32;
    pub Level3: i32;
    pub HisVarCount: i32;
    pub HisVarType: Vec<i32>;
    pub HisVarValue: Vec<i32>;
    pub HisVarNato: Vec<i32>;
    pub HisVarSmall: Vec<i32>;
    pub UseModelCounter: i32;
    pub LibIdClass LibId;
    pub LibIdClass OffLibId;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  this.Name);
      info.AddValue("Type", this.Type);
      info.AddValue("ID", this.ID);
      info.AddValue("Counter", this.Counter);
      info.AddValue("CounterString",  this.CounterString);
      info.AddValue("TempRegime", this.TempRegime);
      info.AddValue("CommanderName",  this.CommanderName);
      info.AddValue("CommanderFileName",  this.CommanderFileName);
      info.AddValue("OverdrawFileName",  this.OverdrawFileName);
      info.AddValue("DeckCardCounter", this.DeckCardCounter);
      info.AddValue("DeckCard",  this.DeckCard);
      info.AddValue("DeckChance",  this.DeckChance);
      info.AddValue("AutoEventCounter", this.AutoEventCounter);
      info.AddValue("AutoEvent",  this.AutoEvent);
      info.AddValue("AutoChance",  this.AutoChance);
      info.AddValue("HandCardCounter", this.HandCardCounter);
      info.AddValue("HandCard",  this.HandCard);
      info.AddValue("PP", this.PP);
      info.AddValue("Model", this.Model);
      info.AddValue("Red", this.Red);
      info.AddValue("Green", this.Green);
      info.AddValue("Blue", this.Blue);
      info.AddValue("SubParts",  this.SubParts);
      info.AddValue("Designation",  this.Designation);
      info.AddValue("NoSplit", this.NoSplit);
      info.AddValue("CombatMod", this.CombatMod);
      info.AddValue("MoraleMod", this.MoraleMod);
      info.AddValue("StaffSize", this.StaffSize);
      info.AddValue("ModelMaster", this.ModelMaster);
      info.AddValue("NameCounter", this.NameCounter);
      info.AddValue("NameCounterBackup", this.NameCounterBackup);
      info.AddValue("Fixed", this.Fixed);
      info.AddValue("Pool", this.Pool);
      info.AddValue("Descript",  this.Descript);
      info.AddValue("Xp", this.Xp);
      info.AddValue("TempVar1", this.TempVar1);
      info.AddValue("TempVar2", this.TempVar2);
      info.AddValue("TempVar3", this.TempVar3);
      info.AddValue("TempVar4", this.TempVar4);
      info.AddValue("TempVar5", this.TempVar5);
      info.AddValue("StartSize", this.StartSize);
      info.AddValue("AIList", this.AIlist);
      info.AddValue("Level1", this.Level1);
      info.AddValue("Level2", this.Level2);
      info.AddValue("Level3", this.Level3);
      info.AddValue("HisVarCount", this.HisVarCount);
      info.AddValue("HisVarType",  this.HisVarType);
      info.AddValue("HisVarValue",  this.HisVarValue);
      info.AddValue("HisVarNato",  this.HisVarNato);
      info.AddValue("HisVarSmall",  this.HisVarSmall);
      info.AddValue("People", this.People);
      info.AddValue("MaxPresent", this.MaxPresent);
      info.AddValue("PercentOldName", this.PercentOldName);
      info.AddValue("UseRomans", this.UseRomans);
      info.AddValue("UseModelCounter", this.UseModelCounter);
      info.AddValue("UsePeopleGfx", this.UsePeopleGfx);
      info.AddValue("LibId",  this.LibId);
      info.AddValue("OffLibId",  this.OffLibId);
      info.AddValue("SmallGfx", this.SmallGfx);
      info.AddValue("DesignationSmall",  this.DesignationSmall);
      info.AddValue("Battlegroup", this.BattleGroup);
    }

    protected HistoricalUnitClass(SerializationInfo info, StreamingContext context)
    {
      this.DeckCard = new int[2];
      this.DeckChance = new int[2];
      this.AutoEvent = new int[2];
      this.AutoChance = new int[2];
      this.HandCard = new int[2];
      this.SubParts = new int[10];
      this.Designation = new int[10];
      this.DesignationSmall = new int[10];
      this.HisVarType = new int[1];
      this.HisVarValue = new int[1];
      this.HisVarNato = new int[1];
      this.HisVarSmall = new int[1];
      this.Name = info.GetString(nameof (Name));
      this.Type = info.GetInt32(nameof (Type));
      this.ID = info.GetInt32(nameof (ID));
      this.Counter = info.GetInt32(nameof (Counter));
      this.CounterString = info.GetString(nameof (CounterString));
      if (DrawMod.TGame.Data.Version < 158)
      {
        try
        {
          this.TempRegime = info.GetInt32(nameof (TempRegime));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.TempRegime = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.StartSize = info.GetInt32(nameof (StartSize));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.StartSize = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.OverdrawFileName = info.GetString(nameof (OverdrawFileName));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.OverdrawFileName = "";
          ProjectData.ClearProjectError();
        }
        try
        {
          this.CommanderName = info.GetString(nameof (CommanderName));
          this.CommanderFileName = info.GetString(nameof (CommanderFileName));
          this.DeckCardCounter = info.GetInt32(nameof (DeckCardCounter));
          this.DeckCard = new int[this.DeckCardCounter + 1];
          this.DeckChance = new int[this.DeckCardCounter + 1];
          this.DeckCard = (int[]) info.GetValue(nameof (DeckCard), this.DeckCard.GetType());
          this.DeckChance = (int[]) info.GetValue(nameof (DeckChance), this.DeckChance.GetType());
          this.AutoEventCounter = info.GetInt32(nameof (AutoEventCounter));
          this.AutoEvent = new int[this.AutoEventCounter + 1];
          this.AutoChance = new int[this.AutoEventCounter + 1];
          this.AutoEvent = (int[]) info.GetValue(nameof (AutoEvent), this.AutoEvent.GetType());
          this.AutoChance = (int[]) info.GetValue(nameof (AutoChance), this.AutoChance.GetType());
          this.HandCardCounter = info.GetInt32(nameof (HandCardCounter));
          this.HandCard = new int[this.HandCardCounter + 1];
          this.HandCard = (int[]) info.GetValue(nameof (HandCard), this.HandCard.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.DeckCardCounter = -1;
          this.AutoEventCounter = -1;
          this.HandCardCounter = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.PP = info.GetInt32(nameof (PP));
          this.Model = info.GetBoolean(nameof (Model));
          this.Red = info.GetInt32(nameof (Red));
          this.Green = info.GetInt32(nameof (Green));
          this.Blue = info.GetInt32(nameof (Blue));
          this.SubParts = (int[]) info.GetValue(nameof (SubParts), this.SubParts.GetType());
          this.Designation = (int[]) info.GetValue(nameof (Designation), this.Designation.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.PP = 0;
          this.Model = false;
          this.Red = 0;
          this.Green = 0;
          this.Blue = 0;
          let mut index: i32 =  0;
          do
          {
            this.SubParts[index] = -1;
            this.Designation[index] = -1;
            index += 1;
          }
          while (index <= 9);
          ProjectData.ClearProjectError();
        }
        try
        {
          this.CombatMod = info.GetInt32(nameof (CombatMod));
          this.MoraleMod = info.GetInt32(nameof (MoraleMod));
          this.StaffSize = info.GetInt32(nameof (StaffSize));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.CombatMod = 0;
          this.MoraleMod = 0;
          this.StaffSize = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.ModelMaster = info.GetInt32(nameof (ModelMaster));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.ModelMaster = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.Fixed = info.GetBoolean(nameof (Fixed));
          this.NameCounter = info.GetInt32(nameof (NameCounter));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.Fixed = false;
          this.NameCounter = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.NameCounterBackup = info.GetInt32(nameof (NameCounterBackup));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.NameCounterBackup = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.Pool = info.GetBoolean(nameof (Pool));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.Pool = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.NoSplit = info.GetBoolean(nameof (NoSplit));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.NoSplit = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.Descript = info.GetString(nameof (Descript));
          this.Xp = info.GetInt32(nameof (Xp));
          this.TempVar1 = info.GetInt32(nameof (TempVar1));
          this.TempVar2 = info.GetInt32(nameof (TempVar2));
          this.TempVar3 = info.GetInt32(nameof (TempVar3));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.Descript = "";
          this.Xp = 0;
          this.TempVar1 = 0;
          this.TempVar2 = 0;
          this.TempVar3 = 0;
          ProjectData.ClearProjectError();
        }
        this.TempVar4 = 0;
        this.TempVar5 = 0;
      }
      else
      {
        this.TempRegime = info.GetInt32(nameof (TempRegime));
        this.StartSize = info.GetInt32(nameof (StartSize));
        this.OverdrawFileName = info.GetString(nameof (OverdrawFileName));
        this.CommanderName = info.GetString(nameof (CommanderName));
        if (Information.IsNothing( this.CommanderName))
          this.CommanderName = "";
        this.CommanderFileName = info.GetString(nameof (CommanderFileName));
        this.DeckCardCounter = info.GetInt32(nameof (DeckCardCounter));
        if (this.DeckCardCounter > -1)
        {
          this.DeckCard = new int[this.DeckCardCounter + 1];
          this.DeckChance = new int[this.DeckCardCounter + 1];
        }
        else
        {
          this.DeckCard = new int[1];
          this.DeckChance = new int[1];
        }
        this.DeckCard = (int[]) info.GetValue(nameof (DeckCard), this.DeckCard.GetType());
        this.DeckChance = (int[]) info.GetValue(nameof (DeckChance), this.DeckChance.GetType());
        this.AutoEventCounter = info.GetInt32(nameof (AutoEventCounter));
        if (this.AutoEventCounter > -1)
        {
          this.AutoEvent = new int[this.AutoEventCounter + 1];
          this.AutoChance = new int[this.AutoEventCounter + 1];
        }
        else
        {
          this.AutoEvent = new int[1];
          this.AutoChance = new int[1];
        }
        this.AutoEvent = (int[]) info.GetValue(nameof (AutoEvent), this.AutoEvent.GetType());
        this.AutoChance = (int[]) info.GetValue(nameof (AutoChance), this.AutoChance.GetType());
        this.HandCardCounter = info.GetInt32(nameof (HandCardCounter));
        this.HandCard = new int[this.HandCardCounter + 1];
        this.HandCard = (int[]) info.GetValue(nameof (HandCard), this.HandCard.GetType());
        this.PP = info.GetInt32(nameof (PP));
        this.Model = info.GetBoolean(nameof (Model));
        this.Red = info.GetInt32(nameof (Red));
        this.Green = info.GetInt32(nameof (Green));
        this.Blue = info.GetInt32(nameof (Blue));
        this.SubParts = (int[]) info.GetValue(nameof (SubParts), this.SubParts.GetType());
        this.Designation = (int[]) info.GetValue(nameof (Designation), this.Designation.GetType());
        this.CombatMod = info.GetInt32(nameof (CombatMod));
        this.MoraleMod = info.GetInt32(nameof (MoraleMod));
        this.StaffSize = info.GetInt32(nameof (StaffSize));
        this.ModelMaster = info.GetInt32(nameof (ModelMaster));
        this.Fixed = info.GetBoolean(nameof (Fixed));
        this.NameCounter = info.GetInt32(nameof (NameCounter));
        try
        {
          this.NameCounterBackup = info.GetInt32(nameof (NameCounterBackup));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.NameCounterBackup = -1;
          ProjectData.ClearProjectError();
        }
        this.Pool = info.GetBoolean(nameof (Pool));
        this.NoSplit = info.GetBoolean(nameof (NoSplit));
        this.Descript = info.GetString(nameof (Descript));
        this.Xp = info.GetInt32(nameof (Xp));
        this.TempVar1 = info.GetInt32(nameof (TempVar1));
        this.TempVar2 = info.GetInt32(nameof (TempVar2));
        this.TempVar3 = info.GetInt32(nameof (TempVar3));
      }
      if (DrawMod.TGame.Data.Version < 180)
      {
        this.AIlist = info.GetInt32("AIList");
        this.Level1 = info.GetInt32(nameof (Level1));
        this.Level2 = info.GetInt32(nameof (Level2));
        this.Level3 = info.GetInt32(nameof (Level3));
      }
      else
      {
        try
        {
          this.AIlist = info.GetInt32("AIList");
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.AIlist = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.Level1 = info.GetInt32(nameof (Level1));
          this.Level2 = info.GetInt32(nameof (Level2));
          this.Level3 = info.GetInt32(nameof (Level3));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.Level1 = 0;
          this.Level2 = 0;
          this.Level3 = 0;
          ProjectData.ClearProjectError();
        }
      }
      if (DrawMod.TGame.Data.Version > 200)
      {
        try
        {
          this.TempVar4 = info.GetInt32(nameof (TempVar4));
          this.TempVar5 = info.GetInt32(nameof (TempVar5));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.TempVar4 = 0;
          this.TempVar5 = 0;
          ProjectData.ClearProjectError();
        }
      }
      this.CommanderSpriteID = -1;
      try
      {
        this.HisVarCount = info.GetInt32(nameof (HisVarCount));
        this.HisVarType = new int[this.HisVarCount + 1];
        this.HisVarValue = new int[this.HisVarCount + 1];
        this.HisVarNato = new int[this.HisVarCount + 1];
        this.HisVarType = (int[]) info.GetValue(nameof (HisVarType), this.HisVarType.GetType());
        this.HisVarValue = (int[]) info.GetValue(nameof (HisVarValue), this.HisVarValue.GetType());
        this.HisVarNato = (int[]) info.GetValue(nameof (HisVarNato), this.HisVarNato.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.HisVarCount = -1;
        this.HisVarType = new int[1];
        this.HisVarValue = new int[1];
        this.HisVarNato = new int[1];
        ProjectData.ClearProjectError();
      }
      try
      {
        this.HisVarSmall = new int[this.HisVarCount + 1];
        this.HisVarSmall = (int[]) info.GetValue(nameof (HisVarSmall), this.HisVarNato.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.HisVarSmall = new int[this.HisVarCount + 1];
        let mut hisVarCount: i32 =  this.HisVarCount;
        for (let mut index: i32 =  0; index <= hisVarCount; index += 1)
          this.HisVarSmall[index] = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.People = info.GetInt32(nameof (People));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.People = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.MaxPresent = info.GetInt32(nameof (MaxPresent));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.MaxPresent = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.PercentOldName = info.GetInt32(nameof (PercentOldName));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.PercentOldName = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.UseRomans = info.GetBoolean(nameof (UseRomans));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.UseRomans = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.UseModelCounter = info.GetInt32(nameof (UseModelCounter));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.UseModelCounter = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.UsePeopleGfx = info.GetInt32(nameof (UsePeopleGfx));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.UsePeopleGfx = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.LibId = LibIdClass::new();
        this.LibId = (LibIdClass) info.GetValue(nameof (LibId), this.LibId.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.LibId = LibIdClass::new();
        ProjectData.ClearProjectError();
      }
      try
      {
        this.OffLibId = LibIdClass::new();
        this.OffLibId = (LibIdClass) info.GetValue(nameof (OffLibId), this.OffLibId.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.OffLibId = LibIdClass::new();
        ProjectData.ClearProjectError();
      }
      try
      {
        this.SmallGfx = info.GetInt32(nameof (SmallGfx));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.SmallGfx = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.DesignationSmall = (int[]) info.GetValue(nameof (DesignationSmall), this.DesignationSmall.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.DesignationSmall = new int[10];
        ProjectData.ClearProjectError();
      }
      try
      {
        this.BattleGroup = info.GetInt32("Battlegroup");
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.BattleGroup = 0;
        ProjectData.ClearProjectError();
      }
    }

    pub HistoricalUnitClass()
    {
      this.DeckCard = new int[2];
      this.DeckChance = new int[2];
      this.AutoEvent = new int[2];
      this.AutoChance = new int[2];
      this.HandCard = new int[2];
      this.SubParts = new int[10];
      this.Designation = new int[10];
      this.DesignationSmall = new int[10];
      this.HisVarType = new int[1];
      this.HisVarValue = new int[1];
      this.HisVarNato = new int[1];
      this.HisVarSmall = new int[1];
      this.Name = "Empty Historical";
      this.Type = 1;
      this.SmallGfx = -1;
      this.CounterString = "";
      this.TempRegime = -1;
      this.NameCounter = 0;
      this.NameCounterBackup = -1;
      this.Fixed = false;
      this.People = -1;
      this.MaxPresent = -1;
      this.BattleGroup = 0;
      this.HisVarCount = -1;
      this.DeckCardCounter = -1;
      this.AutoEventCounter = -1;
      this.HandCardCounter = -1;
      this.OverdrawSpriteID = -1;
      this.CommanderSpriteID = -1;
      this.TempAttack = -1;
      this.TempDefend = -1;
      this.CommanderName = "";
      this.TempStance = -1;
      this.UseRomans = false;
      this.UseModelCounter = -1;
      this.PP = 0;
      this.Descript = "";
      this.Xp = 0;
      this.TempVar1 = 0;
      this.TempVar2 = 0;
      this.TempVar3 = 0;
      this.TempVar4 = 0;
      this.TempVar5 = 0;
      this.Model = false;
      this.Pool = false;
      this.ModelMaster = -1;
      this.UsePeopleGfx = -1;
      this.Red = 0;
      this.Green = 0;
      this.Blue = 0;
      let mut index: i32 =  0;
      do
      {
        this.SubParts[index] = -1;
        this.Designation[index] = -1;
        this.DesignationSmall[index] = -1;
        index += 1;
      }
      while (index <= 9);
      this.PercentOldName = 0;
      this.LibId = LibIdClass::new();
      this.OffLibId = LibIdClass::new();
    }

    pub fn LoadSprites()
    {
      this.CommanderSpriteID = Operators.CompareString(this.CommanderFileName, "", false) == 0 ? -1 : BitmapStore.AddFile(this.CommanderFileName, false);
      if (Operators.CompareString(this.OverdrawFileName, "", false) != 0)
        this.OverdrawSpriteID = BitmapStore.AddFile(this.OverdrawFileName, false);
      else
        this.OverdrawSpriteID = -1;
    }

    pub fn GiveHisVarValue(typ: i32) -> i32
    {
      let mut hisVarCount: i32 =  this.HisVarCount;
      for (let mut index: i32 =  0; index <= hisVarCount; index += 1)
      {
        if (this.HisVarType[index] == typ)
          return this.HisVarValue[index];
      }
      return 0;
    }

    pub fn SetHisVarValue(typ: i32, value: i32, smallGfx: i32)
    {
      let mut hisVarCount: i32 =  this.HisVarCount;
      for (let mut index: i32 =  0; index <= hisVarCount; index += 1)
      {
        if (this.HisVarType[index] == typ)
        {
          this.HisVarValue[index] = value;
          this.HisVarSmall[index] = smallGfx;
          return;
        }
      }
      this += 1.HisVarCount;
      this.HisVarNato = (int[]) Utils.CopyArray((Array) this.HisVarNato, (Array) new int[this.HisVarCount + 1]);
      this.HisVarSmall = (int[]) Utils.CopyArray((Array) this.HisVarSmall, (Array) new int[this.HisVarCount + 1]);
      this.HisVarType = (int[]) Utils.CopyArray((Array) this.HisVarType, (Array) new int[this.HisVarCount + 1]);
      this.HisVarValue = (int[]) Utils.CopyArray((Array) this.HisVarValue, (Array) new int[this.HisVarCount + 1]);
      this.HisVarType[this.HisVarCount] = typ;
      this.HisVarValue[this.HisVarCount] = value;
      this.HisVarNato[this.HisVarCount] = 0;
      this.HisVarSmall[this.HisVarCount] = smallGfx;
    }

    pub fn SetHisVarValue(typ: i32, value: i32)
    {
      let mut hisVarCount: i32 =  this.HisVarCount;
      index: i32;
      for (index = 0; index <= hisVarCount; index += 1)
      {
        if (this.HisVarType[index] == typ)
        {
          this.HisVarValue[index] = value;
          return;
        }
      }
      this += 1.HisVarCount;
      this.HisVarNato = (int[]) Utils.CopyArray((Array) this.HisVarNato, (Array) new int[index + 1]);
      this.HisVarSmall = (int[]) Utils.CopyArray((Array) this.HisVarSmall, (Array) new int[index + 1]);
      this.HisVarType = (int[]) Utils.CopyArray((Array) this.HisVarType, (Array) new int[index + 1]);
      this.HisVarValue = (int[]) Utils.CopyArray((Array) this.HisVarValue, (Array) new int[index + 1]);
      this.HisVarType[this.HisVarCount] = typ;
      this.HisVarValue[this.HisVarCount] = value;
      this.HisVarNato[this.HisVarCount] = 0;
      this.HisVarSmall[this.HisVarCount] = -1;
    }

    pub fn ReplaceSprite1(s: String)
    {
      if (Operators.CompareString(s, "", false) == 0)
      {
        this.CommanderFileName = "";
        if (this.CommanderSpriteID <= -1)
          return;
        BitmapStore.RemoveBitmapNr(this.CommanderSpriteID);
        this.CommanderSpriteID = -1;
      }
      else if (Operators.CompareString(this.CommanderFileName, "", false) == 0)
      {
        this.CommanderFileName = s;
        this.CommanderSpriteID = BitmapStore.AddFile(s, false);
      }
      else
      {
        this.CommanderFileName = s;
        this.CommanderSpriteID = BitmapStore.ReloadFile(this.CommanderSpriteID, this.CommanderFileName);
      }
    }

    pub fn ReplaceSprite2(s2: String)
    {
      if (Operators.CompareString(s2, "", false) == 0)
      {
        this.OverdrawFileName = "";
        if (this.OverdrawSpriteID <= -1)
          return;
        BitmapStore.RemoveBitmapNr(this.OverdrawSpriteID);
        this.OverdrawSpriteID = -1;
      }
      else if (Operators.CompareString(this.OverdrawFileName, "", false) == 0)
      {
        this.OverdrawFileName = s2;
        this.OverdrawSpriteID = BitmapStore.AddFile(s2, false);
      }
      else
      {
        this.OverdrawFileName = s2;
        this.OverdrawSpriteID = BitmapStore.ReloadFile(this.OverdrawSpriteID, this.OverdrawFileName);
      }
    }

    pub HistoricalUnitClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (HistoricalUnitClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }
  }
}
