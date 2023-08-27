// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PrefsWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class PrefsWindowClass : WindowClass
  {
     LocNr: i32;
     BNameId: i32;
     BNameTextId: i32;
     B1Id: i32;
     B1TextId: i32;
     saveid: i32;
     quitid: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     B6Id: i32;
     B6TextId: i32;
     B7Id: i32;
     B7TextId: i32;
     B8Id: i32;
     B8TextId: i32;
     B9Id: i32;
     B9TextId: i32;
     B10Id: i32;
     B10TextId: i32;
     B11Id: i32;
     B11TextId: i32;
     B12Id: i32;
     B12TextId: i32;
     B13Id: i32;
     B13TextId: i32;
     B14Id: i32;
     B14TextId: i32;
     B15Id: i32;
     B15TextId: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
     detailnr: i32;
     regnr: i32;

    pub PrefsWindowClass( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 = -1, let mut sy: i32 = -1)
      : base( tGame, 1024, 200, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.regnr = this.game.Data.Turn;
      this.detailnr = -1;
      this.fixshade = true;
      this.dostuff();
    }

     void dostuff()
    {
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
        this.RemoveSubPart(this.Text2Id);
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.B1Id > 0)
        this.RemoveSubPart(this.B1Id);
      if (this.B1TextId > 0)
        this.RemoveSubPart(this.B1TextId);
      if (this.B2Id > 0)
        this.RemoveSubPart(this.B2Id);
      if (this.B2TextId > 0)
        this.RemoveSubPart(this.B2TextId);
      if (this.B3Id > 0)
        this.RemoveSubPart(this.B3Id);
      if (this.B3TextId > 0)
        this.RemoveSubPart(this.B3TextId);
      if (this.B4Id > 0)
        this.RemoveSubPart(this.B4Id);
      if (this.B4TextId > 0)
        this.RemoveSubPart(this.B4TextId);
      if (this.B5Id > 0)
        this.RemoveSubPart(this.B5Id);
      if (this.B5TextId > 0)
        this.RemoveSubPart(this.B5TextId);
      if (this.B6Id > 0)
        this.RemoveSubPart(this.B6Id);
      if (this.B6TextId > 0)
        this.RemoveSubPart(this.B6TextId);
      if (this.B7Id > 0)
        this.RemoveSubPart(this.B7Id);
      if (this.B7TextId > 0)
        this.RemoveSubPart(this.B7TextId);
      if (this.B8Id > 0)
        this.RemoveSubPart(this.B8Id);
      if (this.B8TextId > 0)
        this.RemoveSubPart(this.B8TextId);
      if (this.B9Id > 0)
        this.RemoveSubPart(this.B9Id);
      if (this.B9TextId > 0)
        this.RemoveSubPart(this.B9TextId);
      if (this.B10Id > 0)
        this.RemoveSubPart(this.B10Id);
      if (this.B10TextId > 0)
        this.RemoveSubPart(this.B10TextId);
      if (this.B11Id > 0)
        this.RemoveSubPart(this.B11Id);
      if (this.B11TextId > 0)
        this.RemoveSubPart(this.B11TextId);
      if (this.B12Id > 0)
        this.RemoveSubPart(this.B12Id);
      if (this.B12TextId > 0)
        this.RemoveSubPart(this.B12TextId);
      if (this.B13Id > 0)
        this.RemoveSubPart(this.B13Id);
      if (this.B13TextId > 0)
        this.RemoveSubPart(this.B13TextId);
      if (this.B14Id > 0)
        this.RemoveSubPart(this.B14Id);
      if (this.B14TextId > 0)
        this.RemoveSubPart(this.B14TextId);
      if (this.B15Id > 0)
        this.RemoveSubPart(this.B15Id);
      if (this.B15TextId > 0)
        this.RemoveSubPart(this.B15TextId);
      if (this.saveid > 0)
        this.RemoveSubPart(this.saveid);
      if (this.quitid > 0)
        this.RemoveSubPart(this.quitid);
      this.NewBackGroundAndClearAll(1024, 200, -1);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (!this.game.EditObj.SoundOn)
      {
        let mut tsubpart: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 10);
        this.B1Id = this.AddSubPart( tsubpart, 140, 10, 35, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 10);
        this.B1Id = this.AddSubPart( tsubpart, 140, 10, 35, 35, 1);
      }
      let mut tsubpart1: SubPartClass =  new ATTextPartClass("Sound Effects", this.game.VicFont2, 200, 24, false);
      this.B1TextId = this.AddSubPart( tsubpart1, 190, 19, 200, 24, 0);
      if (!this.game.EditObj.PrefShowFOW)
      {
        let mut tsubpart2: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 45);
        this.B2Id = this.AddSubPart( tsubpart2, 140, 45, 35, 35, 1);
      }
      else
      {
        let mut tsubpart3: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 45);
        this.B2Id = this.AddSubPart( tsubpart3, 140, 45, 35, 35, 1);
      }
      let mut tsubpart4: SubPartClass =  new ATTextPartClass("Show FOW", this.game.VicFont2, 200, 24, false);
      this.B2TextId = this.AddSubPart( tsubpart4, 190, 54, 200, 24, 0);
      if (!this.game.EditObj.HexRasterOn)
      {
        let mut tsubpart5: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 115);
        this.B4Id = this.AddSubPart( tsubpart5, 140, 115, 35, 35, 1);
      }
      else
      {
        let mut tsubpart6: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 115);
        this.B4Id = this.AddSubPart( tsubpart6, 140, 115, 35, 35, 1);
      }
      let mut tsubpart7: SubPartClass =  new ATTextPartClass("Raster Grid", this.game.VicFont2, 200, 24, false);
      this.B4TextId = this.AddSubPart( tsubpart7, 190, 124, 200, 24, 0);
      if (!this.game.EditObj.IntroSoundOn)
      {
        let mut tsubpart8: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 80);
        this.B8Id = this.AddSubPart( tsubpart8, 140, 80, 35, 35, 1);
      }
      else
      {
        let mut tsubpart9: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 80);
        this.B8Id = this.AddSubPart( tsubpart9, 140, 80, 35, 35, 1);
      }
      let mut tsubpart10: SubPartClass =  new ATTextPartClass("Background Music", this.game.VicFont2, 200, 24, false);
      this.B8TextId = this.AddSubPart( tsubpart10, 190, 87, 200, 24, 0);
      if (!this.game.EditObj.RegimeColoring)
      {
        let mut tsubpart11: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 150);
        this.B12Id = this.AddSubPart( tsubpart11, 140, 150, 35, 35, 1);
      }
      else
      {
        let mut tsubpart12: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 140, bby: 150);
        this.B12Id = this.AddSubPart( tsubpart12, 140, 150, 35, 35, 1);
      }
      let mut tsubpart13: SubPartClass =  new ATTextPartClass("Hex Coloring", this.game.VicFont2, 200, 24, false);
      this.B12TextId = this.AddSubPart( tsubpart13, 190, 157, 200, 24, 0);
      if (!this.game.Data.PBEM | this.game.EditObj.InEditor)
      {
        let mut tsubpart14: SubPartClass =  new SteveButtonPartClass(this.game.BUTTONSAVE, tDescript: "Save", tBackbitmap: ( this.OwnBitmap), bbx: 740, bby: 90);
        this.saveid = this.AddSubPart( tsubpart14, 740, 140, 32, 32, 1);
        let mut tsubpart15: SubPartClass =  new ATTextPartClass("Save", this.game.VicFont2, 200, 24, false);
        this.Text2Id = this.AddSubPart( tsubpart15, 790, 147, 200, 24, 0);
      }
      let mut tsubpart16: SubPartClass =  new SteveButtonPartClass(this.game.BUTTONQUIT, tDescript: "Quit", tBackbitmap: ( this.OwnBitmap), bbx: 740, bby: 50);
      this.quitid = this.AddSubPart( tsubpart16, 740, 105, 32, 32, 1);
      let mut tsubpart17: SubPartClass =  new ATTextPartClass("Quit", this.game.VicFont2, 200, 24, false);
      this.Text1Id = this.AddSubPart( tsubpart17, 790, 112, 200, 24, 0);
      if (this.game.Data.Round <= 1)
      {
        if (!this.game.EditObj.CombatSim)
        {
          let mut tsubpart18: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 115);
          this.B6Id = this.AddSubPart( tsubpart18, 440, 115, 35, 35, 1);
        }
        else
        {
          let mut tsubpart19: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 115);
          this.B6Id = this.AddSubPart( tsubpart19, 440, 115, 35, 35, 1);
        }
        let mut tsubpart20: SubPartClass =  new ATTextPartClass("Combat Sim", this.game.VicFont2, 200, 24, false);
        this.B6TextId = this.AddSubPart( tsubpart20, 490, 122, 200, 24, 0);
      }
      if (!this.game.EditObj.ShowBase)
      {
        let mut tsubpart21: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 150);
        this.B7Id = this.AddSubPart( tsubpart21, 440, 150, 35, 35, 1);
      }
      else
      {
        let mut tsubpart22: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 150);
        this.B7Id = this.AddSubPart( tsubpart22, 440, 150, 35, 35, 1);
      }
      let mut tsubpart23: SubPartClass =  new ATTextPartClass("Show Location Bases", this.game.VicFont2, 200, 24, false);
      this.B7TextId = this.AddSubPart( tsubpart23, 490, 159, 200, 24, 0);
      if (this.game.EditObj.HideAS)
      {
        let mut tsubpart24: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 80);
        this.B5Id = this.AddSubPart( tsubpart24, 440, 80, 35, 35, 1);
      }
      else
      {
        let mut tsubpart25: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 80);
        this.B5Id = this.AddSubPart( tsubpart25, 440, 80, 35, 35, 1);
      }
      let mut tsubpart26: SubPartClass =  new ATTextPartClass("Show Extra Hex Info", this.game.VicFont2, 200, 24, false);
      this.B5TextId = this.AddSubPart( tsubpart26, 490, 84, 200, 24, 0);
      if (!this.game.EditObj.AutoSave)
      {
        let mut tsubpart27: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 115);
        this.B9Id = this.AddSubPart( tsubpart27, 440, 10, 35, 35, 1);
      }
      else
      {
        let mut tsubpart28: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 115);
        this.B9Id = this.AddSubPart( tsubpart28, 440, 10, 35, 35, 1);
      }
      let mut tsubpart29: SubPartClass =  new ATTextPartClass("Auto Save", this.game.VicFont2, 200, 24, false);
      this.B9TextId = this.AddSubPart( tsubpart29, 490, 17, 200, 24, 0);
      if (!this.game.EditObj.ShowLabel)
      {
        let mut tsubpart30: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 740, bby: 45);
        this.B10Id = this.AddSubPart( tsubpart30, 740, 45, 35, 35, 1);
      }
      else
      {
        let mut tsubpart31: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 740, bby: 45);
        this.B10Id = this.AddSubPart( tsubpart31, 740, 45, 35, 35, 1);
      }
      let mut tsubpart32: SubPartClass =  new ATTextPartClass("Show Hex Labels", this.game.VicFont2, 200, 24, false);
      this.B10TextId = this.AddSubPart( tsubpart32, 790, 55, 200, 24, 0);
      if (!this.game.EditObj.Screenshoton)
      {
        let mut tsubpart33: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 740, bby: 130);
        this.B13Id = this.AddSubPart( tsubpart33, 740, 10, 35, 35, 1);
      }
      else
      {
        let mut tsubpart34: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 740, bby: 130);
        this.B13Id = this.AddSubPart( tsubpart34, 740, 10, 35, 35, 1);
      }
      let mut tsubpart35: SubPartClass =  new ATTextPartClass("Make screenshots", this.game.VicFont2, 200, 24, false);
      this.B13TextId = this.AddSubPart( tsubpart35, 790, 17, 200, 24, 0);
      if (this.game.Data.Round > 0)
      {
        if (!this.game.EditObj.TownInfo)
        {
          let mut tsubpart36: SubPartClass =  new SteveButtonPartClass(this.game.CANCELBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 45);
          this.B15Id = this.AddSubPart( tsubpart36, 440, 45, 35, 35, 1);
        }
        else
        {
          let mut tsubpart37: SubPartClass =  new SteveButtonPartClass(this.game.OKBALL, tBackbitmap: ( this.OwnBitmap), bbx: 440, bby: 45);
          this.B15Id = this.AddSubPart( tsubpart37, 440, 45, 35, 35, 1);
        }
        let mut tsubpart38: SubPartClass =  new ATTextPartClass("Town Info", this.game.VicFont2, 200, 24, false);
        this.B15TextId = this.AddSubPart( tsubpart38, 490, 52, 200, 24, 0);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = this.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 = this.SubPartID[index1];
            if (num1 == this.saveid)
            {
              str: String;
              if (this.game.Data.Round == 0)
              {
                tinitdir: String = this.game.AppPath + "scenarios\\";
                if (!Information.IsNothing( this.game.Data.ScenarioDir))
                {
                  if (this.game.Data.ScenarioDir.Length > 1)
                    tinitdir = tinitdir.Replace("scenarios", this.game.Data.ScenarioDir);
                  else if (this.game.ModScenarioDir.Length > 1)
                    tinitdir = tinitdir.Replace("scenarios", this.game.ModScenarioDir);
                }
                else if (this.game.ModScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", this.game.ModScenarioDir);
                let mut eventCounter: i32 = this.game.Data.EventCounter;
                for (let mut index2: i32 = 0; index2 <= eventCounter; index2 += 1)
                  this.game.Data.EventObj[index2].Blocked = false;
                str = this.game.HandyFunctionsObj.SaveSomething("SE1 Scenario file (*.se1)|*.se1|SE1 Map file(*.se1map)|*.se1map|SE1 Master file(*.se1master)|*.se1master|SE1 Event library(*.se1evlib)|*.se1evlib|SE1 Troops&Equipment library(*.se1troops)|*.se1troops|SE1 Historical library(*.se1his)|*.se1his|SE1 Officer Card Library(*.se1offcard)|*.se1offcard|SE1 Officer library(*.se1off)|*.se1off", "Give save name...", tinitdir, false);
              }
              else
                str = this.game.HandyFunctionsObj.SaveSomething("SE1 Scenario file (*.se1)|*.se1|SE1 Map file(*.se1map)|*.se1map|SE1 Master file(*.se1master)|*.se1master|SE1 Event library(*.se1evlib)|*.se1evlib|SE1 Troops&Equipment library(*.se1troops)|*.se1troops|SE1 Historical library(*.se1his)|*.se1his|SE1 Officer Card Library(*.se1offcard)|*.se1offcard|SE1 Officer library(*.se1off)|*.se1off", "Give save name...", this.game.AppPath_SAVEGAMES, false);
              if (Strings.Len(str) < 2)
              {
                let mut num2: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                this.game.Data.serialize(str);
                this.game.HandyFunctionsObj.ZipFile(str);
                windowReturnClass.SetFlag(true);
                let mut num3: i32 =  Interaction.MsgBox( "Game has been saved", Title: ( "Shadow Empire : Planetary Conquest"));
              }
            }
            else if (num1 == this.quitid)
            {
              if (Interaction.MsgBox( "Are you Sure?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              {
                this.game.Data = DataClass::new();
                this.game.EditObj = new EditClass(this.game.AppPath + "editobj.txt");
                this.game.EditObj.ShowInitialMenu = true;
                windowReturnClass.AddCommand(3, 1);
                windowReturnClass.SetFlag(true);
              }
            }
            else if (num1 == this.B11Id)
            {
              let mut x1: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give X coord to show extra history log")));
              if (x1 < 0 | x1 > this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
                return windowReturnClass;
              let mut y1: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Y coord to show extra history log")));
              if (y1 < 0 | y1 > this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
                return windowReturnClass;
              str: String = Interaction.InputBox("Give text to add to history log of other players. ");
              if (Strings.Len(str) < 1)
                return windowReturnClass;
              this += 1.game.Data.StepNr;
              this.game.HandyFunctionsObj.HistoryAddHex(x1, y1, this.game.EditObj.MapSelected, this.game.Data.Turn, 2, 0, 0, infostring: str);
            }
            else
            {
              if (num1 == this.B1Id)
              {
                this.game.EditObj.SoundOn = !this.game.EditObj.SoundOn;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B2Id)
              {
                this.game.EditObj.PrefShowFOW = !this.game.EditObj.PrefShowFOW;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(1, 3);
                windowReturnClass.AddCommand(2, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B3Id)
              {
                this.game.EditObj.AutoCombat = !this.game.EditObj.AutoCombat;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B14Id)
              {
                if (this.game.EditObj.Layout == 0)
                  this.game.EditObj.Layout = 1;
                else
                  this.game.EditObj.Layout = 0;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                this.game.EditObj.CameFrom = 45;
                windowReturnClass.AddCommand(3, 3);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B12Id)
              {
                this.game.EditObj.RegimeColoring = !this.game.EditObj.RegimeColoring;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(4, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B15Id)
              {
                this.game.EditObj.TownInfo = !this.game.EditObj.TownInfo;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(4, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == 4444)
              {
                let mut num4: i32 =  Math.Round(Conversion.Int( (this.game.ScreenWidth - 200) / 53.0));
                let mut num5: i32 =  Math.Round(Conversion.Int( (this.game.ScreenWidth - 200) / 106.0));
                let mut num6: i32 =  Math.Round(Conversion.Int( (this.game.ScreenHeight - 265) / 53.0));
                let mut num7: i32 =  Math.Round(Conversion.Int( (this.game.ScreenHeight - 265) / 106.0));
                num8: i32;
                num9: i32;
                if (this.game.EditObj.Zoom == 0)
                {
                  this.game.EditObj.Zoom = 1;
                  this.game.CornerX +=  Math.Round(Conversion.Int( num5 / 2.0));
                  this.game.CornerY +=  Math.Round(Conversion.Int( num7 / 2.0));
                  num8 = 106;
                  num9 = 96;
                }
                else
                {
                  this.game.EditObj.Zoom = 0;
                  this.game.CornerX -=  Math.Round(Conversion.Int( num5 / 2.0));
                  this.game.CornerY -=  Math.Round(Conversion.Int( num7 / 2.0));
                  num8 = 53;
                  num9 = 48;
                }
                if ( this.game.CornerX +  (this.game.ScreenWidth - 200) /  num8 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
                  this.game.CornerX =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth) -  (this.game.ScreenWidth - 200) /  num8);
                if ( this.game.CornerY +  (this.game.ScreenHeight - 256) /  num9 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
                  this.game.CornerY =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) -  (this.game.ScreenHeight - 256) /  num9);
                if (this.game.CornerX < 0)
                  this.game.CornerX = 0;
                if (this.game.CornerY < 0)
                  this.game.CornerY = 0;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(1, 3);
                windowReturnClass.AddCommand(2, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B8Id)
              {
                this.game.EditObj.IntroSoundOn = !this.game.EditObj.IntroSoundOn;
                if (!this.game.EditObj.IntroSoundOn)
                  SoundMod.STopEventBackground();
                else
                  SoundMod.RestartLastBackground( this.game.EditObj);
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B9Id)
              {
                this.game.EditObj.AutoSave = !this.game.EditObj.AutoSave;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B10Id)
              {
                this.game.EditObj.ShowLabel = !this.game.EditObj.ShowLabel;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(4, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B13Id)
              {
                this.game.EditObj.Screenshoton = !this.game.EditObj.Screenshoton;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B4Id)
              {
                this.game.EditObj.HexRasterOn = !this.game.EditObj.HexRasterOn;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(4, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B6Id)
              {
                this.game.EditObj.CombatSim = !this.game.EditObj.CombatSim;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B7Id)
              {
                this.game.EditObj.ShowBase = !this.game.EditObj.ShowBase;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(4, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B5Id)
              {
                this.game.EditObj.HideAS = !this.game.EditObj.HideAS;
                this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
                windowReturnClass.AddCommand(4, 12);
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
