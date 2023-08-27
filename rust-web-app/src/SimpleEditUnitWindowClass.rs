// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleEditUnitWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SimpleEditUnitWindowClass : WindowClass
  {
     listId: i32;
     ListClass listObj;
     loadMapId: i32;
     loadMapIdB: i32;
     importId: i32;
     textId: i32;
     detailnr: i32;
     currentStep: i32;
     standing1: i32;
     standing2: i32;
     ListClass VPListOBj;
     VPListId: i32;
     AddUnitModel: i32;
     AddUnit: i32;
     AddUnitModelB: i32;
     cancelid: i32;
     AddUnitB: i32;
     ChangeCounterUnit: i32;
     ChangeCounterUnitB: i32;
     RemoveUnit: i32;
     MoveUnit: i32;
     MoveUnitB: i32;
     HqUnit: i32;
     HqUnitB: i32;
     RemoveUnitB: i32;
     AddVPIdb: i32;
     SetCommander: i32;
     SetCommanderB: i32;
     changeColor: i32;
     changeColorB: i32;

    pub SimpleEditUnitWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, 300, 9, tDoBorders: 1, tHeaderString: "Units")
    {
      self.detailnr = -1;
      self.game.EditObj.TempHisModelUnit = -1;
      self.game.EditObj.TempHisUnit = -1;
      self.game.EditObj.TempRandom = -1;
      self.DoStuff();
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (nr == 77 & self.game.EditObj.UnitSelected > -1 & self.SubpartNr(self.MoveUnit) > 0)
      {
        windowReturnClass2: WindowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.MoveUnit)] + 1, self.SubPartY[self.SubpartNr(self.MoveUnit)] + 1, 1);
        windowReturnClass2.SetFlag(true);
        return windowReturnClass2;
      }
      if (nr == 72 & self.game.EditObj.UnitSelected > -1 & self.SubpartNr(self.HqUnit) > 0)
      {
        windowReturnClass3: WindowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.HqUnit)] + 1, self.SubPartY[self.SubpartNr(self.HqUnit)] + 1, 1);
        windowReturnClass3.SetFlag(true);
        return windowReturnClass3;
      }
      if (!((nr == 13 | nr == 32) & self.game.EditObj.UnitSelected > -1 & self.SubpartNr(self.cancelid) > 0))
        return windowReturnClass1;
      windowReturnClass4: WindowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.cancelid)] + 1, self.SubPartY[self.SubpartNr(self.cancelid)] + 1, 1);
      windowReturnClass4.SetFlag(true);
      return windowReturnClass4;
    }

    pub fn DoRefresh()
    {
      if (self.game.EditObj.TempHisModelUnit > -1)
      {
        self.game.ProcessingObj.AddNewUnitBasedOnHistorical(self.game.SelectX, self.game.SelectY, 0, self.game.Data.HistoricalUnitObj[self.game.EditObj.TempHisModelUnit].TempRegime, self.game.EditObj.TempHisModelUnit, freePPnoUnit: true, populateUnit: true);
        self.game.EditObj.UnitSelected = self.game.Data.UnitCounter;
        self.game.EditObj.TempHisModelUnit = -1;
      }
      if (self.game.EditObj.TempHisUnit > -1)
      {
        let mut tempRegime: i32 = self.game.Data.HistoricalUnitObj[self.game.EditObj.TempHisUnit].TempRegime;
        self.game.EventRelatedObj.ExecAddHistoricalUnit(self.game.SelectX, self.game.SelectY, self.game.Data.HistoricalUnitObj[self.game.EditObj.TempHisUnit].ID, 0, "");
        self.game.EditObj.UnitSelected = self.game.Data.UnitCounter;
        self.game.EditObj.TempHisUnit = -1;
      }
      if (self.game.EditObj.TempRandom > -1)
      {
        if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].CommanderName.Length > 0)
        {
          LibIdClass libIdClass1 = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].OffLibId.Clone();
          LibIdClass libIdClass2 = self.game.Data.HistoricalUnitObj[self.game.EditObj.TempRandom].LibId.Clone();
          self.game.ProcessingObj.SwapOfficer(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical, self.game.EditObj.TempRandom, self.game.EditObj.UnitSelected);
          self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].OffLibId = libIdClass2;
          self.game.Data.HistoricalUnitObj[self.game.EditObj.TempRandom].LibId = libIdClass1.Clone();
          self.game.Data.HistoricalUnitObj[self.game.EditObj.TempRandom].OffLibId = LibIdClass::new();
          self.game.Data.HistoricalUnitObj[self.game.EditObj.TempRandom].Pool = true;
        }
        else
        {
          LibIdClass libIdClass = self.game.Data.HistoricalUnitObj[self.game.EditObj.TempRandom].LibId.Clone();
          self.game.ProcessingObj.SwapOfficer(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical, self.game.EditObj.TempRandom, self.game.EditObj.UnitSelected);
          self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].OffLibId = libIdClass;
        }
        self.game.EditObj.TempRandom = -1;
      }
      else if (self.game.EditObj.TempRandom == -2 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].CommanderName.Length > 0)
      {
        LibIdClass libIdClass = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].OffLibId.Clone();
        self.game.Data.AddHistoricalUnit();
        self.game.ProcessingObj.SwapOfficer(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical, self.game.Data.HistoricalUnitCounter, self.game.EditObj.UnitSelected);
        self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].OffLibId = LibIdClass::new();
        self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].LibId = libIdClass.Clone();
        self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].Pool = true;
        self.game.EditObj.TempRandom = -1;
      }
      self.DoStuff();
    }

    pub fn DoStuff()
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
      self.ClearMouse();
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight - 350, -1);
      DrawMod.DrawBlock( graphics, 2, 294, DrawMod.TGame.ScreenWidth - 4, 4, 0, 0, 0, 128);
      if (self.VPListId > 0)
      {
        self.RemoveSubPart(self.VPListId);
        self.VPListId = 0;
      }
      if (self.AddUnitModel > 0)
      {
        self.RemoveSubPart(self.AddUnitModel);
        self.AddUnitModel = 0;
      }
      if (self.AddUnit > 0)
      {
        self.RemoveSubPart(self.AddUnit);
        self.AddUnit = 0;
      }
      if (self.RemoveUnit > 0)
      {
        self.RemoveSubPart(self.RemoveUnit);
        self.RemoveUnit = 0;
      }
      if (self.RemoveUnitB > 0)
      {
        self.RemoveSubPart(self.RemoveUnitB);
        self.RemoveUnitB = 0;
      }
      if (self.ChangeCounterUnit > 0)
      {
        self.RemoveSubPart(self.ChangeCounterUnit);
        self.ChangeCounterUnit = 0;
      }
      if (self.ChangeCounterUnitB > 0)
      {
        self.RemoveSubPart(self.ChangeCounterUnitB);
        self.ChangeCounterUnitB = 0;
      }
      if (self.AddUnitModelB > 0)
      {
        self.RemoveSubPart(self.AddUnitModelB);
        self.AddUnitModelB = 0;
      }
      if (self.AddUnitB > 0)
      {
        self.RemoveSubPart(self.AddUnitB);
        self.AddUnitB = 0;
      }
      if (self.AddVPIdb > 0)
      {
        self.RemoveSubPart(self.AddVPIdb);
        self.AddVPIdb = 0;
      }
      if (self.SetCommander > 0)
      {
        self.RemoveSubPart(self.SetCommander);
        self.SetCommander = 0;
      }
      if (self.SetCommanderB > 0)
      {
        self.RemoveSubPart(self.SetCommanderB);
        self.SetCommanderB = 0;
      }
      if (self.MoveUnit > 0)
      {
        self.RemoveSubPart(self.MoveUnit);
        self.MoveUnit = 0;
      }
      if (self.HqUnit > 0)
      {
        self.RemoveSubPart(self.HqUnit);
        self.HqUnit = 0;
      }
      if (self.HqUnitB > 0)
      {
        self.RemoveSubPart(self.HqUnitB);
        self.HqUnitB = 0;
      }
      if (self.MoveUnitB > 0)
      {
        self.RemoveSubPart(self.MoveUnitB);
        self.MoveUnitB = 0;
      }
      if (self.HqUnitB > 0)
      {
        self.RemoveSubPart(self.HqUnitB);
        self.HqUnitB = 0;
      }
      if (self.importId > 0)
      {
        self.RemoveSubPart(self.importId);
        self.importId = 0;
      }
      if (self.cancelid > 0)
      {
        self.RemoveSubPart(self.cancelid);
        self.cancelid = 0;
      }
      if (self.changeColor > 0)
      {
        self.RemoveSubPart(self.changeColor);
        self.changeColor = 0;
      }
      if (self.changeColorB > 0)
      {
        self.RemoveSubPart(self.changeColorB);
        self.changeColorB = 0;
      }
      if (self.standing1 > 0)
      {
        self.RemoveSubPart(self.standing1);
        self.standing1 = 0;
      }
      if (self.standing2 > 0)
      {
        self.RemoveSubPart(self.standing2);
        self.standing2 = 0;
      }
      self.VPListOBj = ListClass::new();
      let mut num2: i32 = num1 + 250;
      let mut y: i32 = 55;
      tDescript: String;
      SubPartClass tsubpart1;
      if (self.game.SelectX > -1 & self.game.SelectY > -1)
      {
        let mut num3: i32 = -1;
        let mut num4: i32 = -1;
        let mut unitCounter: i32 = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter;
        for (let mut index: i32 = 0; index <= unitCounter; index += 1)
        {
          let mut unit: i32 = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitList[index];
          num4 += 1;
          self.VPListOBj.add(index.ToString() + "," + self.game.Data.UnitObj[unit].Name, unit);
          if (self.game.EditObj.UnitSelected == unit)
            num3 = num4;
        }
        if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter == -1)
          self.VPListOBj.add("no units on hex", -1);
        ListClass vpListObj = self.VPListOBj;
        let mut tlistselect: i32 = num3;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        let mut bbx: i32 = 10 + num1;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart2: SubPartClass =  new ListSubPartClass(vpListObj, 10, 200, tlistselect, game, true, "Units in Hex", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local1), bbx: bbx, bby: 52, tMarcStyle: true, overruleFont: ( local2));
        self.VPListId = self.AddSubPart( tsubpart2, 10 + num1, 52, 200, 176, 0);
        bool flag = true;
        if (self.game.EditObj.UnitSelected > -1 && self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical > -1 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].CommanderName.Length > 0)
          flag = false;
        if (flag)
        {
          DrawMod.DrawTextColouredMarc( graphics, "Selected hex:", self.game.MarcFont4, num2 + 400, y + 50, Color.White);
          DrawMod.DrawTextColouredMarc( graphics, self.game.SelectX.ToString() + "," + self.game.SelectY.ToString() + "," + self.game.HandyFunctionsObj.GetHexName(self.game.SelectX, self.game.SelectY, 0), self.game.MarcFont3, num2 + 400, y + 70, Color.White);
        }
        if (self.game.EditObj.UnitSelected > -1)
        {
          DrawMod.DrawTextColouredMarc( graphics, "Selected unit:", self.game.MarcFont4, num2, y, Color.White);
          let mut historical: i32 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical;
          ttext: String = "Unit slot: " + self.game.EditObj.UnitSelected.ToString() + "\r\n";
          if (historical > -1)
            ttext = ttext + "Historical slot: " + historical.ToString() + "\r\n" + "Historical ID: " + self.game.Data.HistoricalUnitObj[historical].ID.ToString() + "\r\n";
          Rectangle trect = Rectangle::new(num2, y, 200, 20);
          self.AddMouse( trect, "Selected unit", ttext);
          DrawMod.DrawTextColouredMarc( graphics, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Name, self.game.MarcFont3, num2, y + 20, Color.White);
          DrawMod.DrawTextColouredMarc( graphics, "Units model:", self.game.MarcFont4, num2, y + 50, Color.White);
          tstring1: String;
          if (historical > -1)
          {
            let mut modelMaster: i32 = self.game.Data.HistoricalUnitObj[historical].ModelMaster;
            if (modelMaster > -1)
            {
              let mut num5: i32 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HistoricalSubPart + 1;
              tstring1 = self.game.Data.HistoricalUnitObj[modelMaster].Name;
              if (!self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
              {
                tstring1 = tstring1 + " (sub-part " + num5.ToString() + ")";
              }
              else
              {
                if (self.game.Data.HistoricalUnitObj[modelMaster].Type == 5)
                  tstring1 += " (lowest level HQ)";
                if (self.game.Data.HistoricalUnitObj[modelMaster].Type == 6)
                  tstring1 += " (medium level HQ)";
                if (self.game.Data.HistoricalUnitObj[modelMaster].Type == 7)
                  tstring1 += " (high level HQ)";
                if (self.game.Data.HistoricalUnitObj[modelMaster].Type == 8)
                  tstring1 += " (supreme level HQ)";
              }
            }
            else
              tstring1 = "Warning! unit not set to a model";
          }
          else
            tstring1 = "Warning! unit is not a historical unit";
          DrawMod.DrawTextColouredMarc( graphics, tstring1, self.game.MarcFont3, num2, y + 70, Color.White);
          DrawMod.DrawTextColouredMarc( graphics, "is HQ:", self.game.MarcFont4, num2 + 400, y, Color.White);
          tstring2: String = "Yes";
          if (!self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
            tstring2 = "No";
          DrawMod.DrawTextColouredMarc( graphics, tstring2, self.game.MarcFont3, num2 + 400, y + 20, Color.White);
          DrawMod.DrawTextColouredMarc( graphics, "Assigned to HQ:", self.game.MarcFont4, num2 + 500, y, Color.White);
          tstring3: String = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HQ != -1 ? self.game.Data.UnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HQ].Name : "None";
          DrawMod.DrawTextColouredMarc( graphics, tstring3, self.game.MarcFont3, num2 + 500, y + 20, Color.White);
          if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical > -1 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].CommanderName.Length > 0)
          {
            DrawMod.DrawTextColouredMarc( graphics, "Commander:", self.game.MarcFont4, num2 + 400, y + 50, Color.White);
            tstring4: String = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].CommanderName;
            let mut people: i32 = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].People;
            if (people > -1)
              tstring4 = tstring4 + " (" + self.game.Data.PeopleObj[people].Name + ")";
            DrawMod.DrawTextColouredMarc( graphics, tstring4, self.game.MarcFont3, num2 + 400, y + 70, Color.White);
          }
        }
        else
        {
          DrawMod.DrawTextColouredMarc( graphics, "Selected unit:", self.game.MarcFont4, num2, y, Color.White);
          DrawMod.DrawTextColouredMarc( graphics, "none", self.game.MarcFont3, num2, y + 20, Color.White);
        }
        if (self.game.EditObj.OrderType >= 1)
        {
          DrawMod.DrawTextColouredMarc( graphics, "Currently executing an order.", self.game.MarcFont4, num2 + 400, y + 50, Color.White);
          tDescript = "Cancel order";
          tsubpart1 =  new TextButtonPartClass("Cancel order", 250, tDescript,  self.OwnBitmap, num2 + 400, y + 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.cancelid = self.AddSubPart( tsubpart1, num2 + 400, y + 100, 250, 35, 1);
        }
        else if (self.game.EditObj.UnitSelected > -1)
        {
          if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
          {
            let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Set Commander", 250, "Change commander",  self.OwnBitmap, num2 + 400, y + 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.SetCommander = self.AddSubPart( tsubpart3, num2 + 400, y + 100, 250, 35, 1);
            let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Change color", 155, "Click to change of: Color HQ.",  self.OwnBitmap, num2 + 190, y + 180, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.changeColor = self.AddSubPart( tsubpart4, num2 + 190, y + 180, 155, 35, 1);
          }
          else
          {
            let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Set Commander", 250, "You can only set commander for a HQ",  self.OwnBitmap, num2 + 400, y + 100, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.SetCommanderB = self.AddSubPart( tsubpart5, num2 + 400, y + 100, 250, 35, 0);
            let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Change color", 155, "You can only change of: Color HQ.",  self.OwnBitmap, num2 + 190, y + 180, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.changeColorB = self.AddSubPart( tsubpart6, num2 + 190, y + 180, 155, 35, 0);
          }
          let mut tsubpart7: SubPartClass =  new TextButtonPartClass("Move Unit [M]", 250, tBackbitmap: ( self.OwnBitmap), bbx: (num2 + 400), bby: (y + 140), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.MoveUnit = self.AddSubPart( tsubpart7, num2 + 400, y + 140, 250, 35, 1);
          let mut tsubpart8: SubPartClass =  new TextButtonPartClass("Set HQ [H]", 250, tBackbitmap: ( self.OwnBitmap), bbx: (num2 + 400), bby: (y + 180), usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.HqUnit = self.AddSubPart( tsubpart8, num2 + 400, y + 180, 250, 35, 1);
          tsubpart1 =  new TextButtonPartClass("Remove unit", 155, "Click to remove this unit.",  self.OwnBitmap, num2, y + 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.RemoveUnit = self.AddSubPart( tsubpart1, num2, y + 100, 155, 35, 1);
          tsubpart1 =  new TextButtonPartClass("Change counter", 155, "Click to change the Number and the Shortname of the selected unit.",  self.OwnBitmap, num2 + 190, y + 140, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.ChangeCounterUnit = self.AddSubPart( tsubpart1, num2 + 190, y + 140, 155, 35, 1);
          tDescript = "Change Standing Order: Retreat Percentage. Will change setting for current unit and all subordinates. 100% is fight till the end. 25% retreat asap. ";
          tsubpart1 =  new TextButtonPartClass("Rtr:" + (100 - self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SODefendPercent).ToString(), 90, tDescript,  self.OwnBitmap, num2 - 240, y + 180, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.standing1 = self.AddSubPart( tsubpart1, num2 - 240, y + 180, 90, 35, 1);
          if (self.game.HandyFunctionsObj.HasUnitAirSF(self.game.EditObj.UnitSelected))
          {
            tDescript = "Change Standing Order: Intercept Percentage. Will change setting for current unit and all subordinates. % specifies the minimum readiness needed to allow intercept missions. ";
            str: String = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SOInterceptRdnStop.ToString();
            if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SOInterceptRdnStop == 100)
              str = "Nev.";
            tsubpart1 =  new TextButtonPartClass("Intc:" + str, 90, tDescript,  self.OwnBitmap, num2 - 140, y + 180, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.standing2 = self.AddSubPart( tsubpart1, num2 - 140, y + 180, 90, 35, 1);
          }
        }
        else
        {
          let mut tsubpart9: SubPartClass =  new TextButtonPartClass("Remove unit", 155, "No unit selected",  self.OwnBitmap, num2, y + 100, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.RemoveUnitB = self.AddSubPart( tsubpart9, num2, y + 100, 155, 35, 0);
          let mut tsubpart10: SubPartClass =  new TextButtonPartClass("Move Unit", 250, "No unit selected",  self.OwnBitmap, num2 + 400, y + 140, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.MoveUnitB = self.AddSubPart( tsubpart10, num2 + 400, y + 140, 250, 35, 0);
          tsubpart1 =  new TextButtonPartClass("Set HQ", 250, "No unit selected",  self.OwnBitmap, num2 + 400, y + 180, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.HqUnitB = self.AddSubPart( tsubpart1, num2 + 400, y + 180, 250, 35, 0);
          tsubpart1 =  new TextButtonPartClass("Change counter", 155, "No unit selected",  self.OwnBitmap, num2 + 190, y + 140, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.ChangeCounterUnitB = self.AddSubPart( tsubpart1, num2 + 190, y + 140, 155, 35, 0);
          tsubpart1 =  new TextButtonPartClass("Change color", 155, "No unit selected",  self.OwnBitmap, num2 + 190, y + 180, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.changeColorB = self.AddSubPart( tsubpart1, num2 + 190, y + 180, 155, 35, 0);
        }
      }
      else
      {
        DrawMod.DrawTextColouredMarc( graphics, "Selected hex:", self.game.MarcFont4, num2, y, Color.White);
        DrawMod.DrawTextColouredMarc( graphics, "None", self.game.MarcFont3, num2, y + 20, Color.White);
      }
      if (self.game.EditObj.OrderType <= 0)
      {
        if (!self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].LandscapeType].IsSea & self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime > -1)
        {
          tsubpart1 =  new TextButtonPartClass("Add unit model", 155, "Click to add a unit to this hex, based on a historical unit model, but a fresh non-yet existing instance of it.",  self.OwnBitmap, num2, y + 140, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.AddUnitModel = self.AddSubPart( tsubpart1, num2, y + 140, 155, 35, 1);
          tDescript = "Click to add a pre-defined historical unit, not yet on map, on the map";
          tsubpart1 =  new TextButtonPartClass("Add unit", 155, tDescript,  self.OwnBitmap, num2, y + 180, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.AddUnit = self.AddSubPart( tsubpart1, num2, y + 180, 155, 35, 1);
        }
        else
        {
          tDescript = "Can only place on friendly hex";
          tsubpart1 =  new TextButtonPartClass("Add unit model", 155, tDescript,  self.OwnBitmap, num2, y + 140, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.AddUnitModelB = self.AddSubPart( tsubpart1, num2, y + 140, 155, 35, 1);
          tsubpart1 =  new TextButtonPartClass("Add unit", 155, tDescript,  self.OwnBitmap, num2, y + 180, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.AddUnitB = self.AddSubPart( tsubpart1, num2, y + 180, 250, 35, 1);
        }
      }
      if (self.game.EditObj.UnitSelected == -1 | self.game.EditObj.OrderType > 0)
      {
        tsubpart1 =  new TextButtonPartClass("Remove unit", 155, tDescript,  self.OwnBitmap, num2, y + 100, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.RemoveUnitB = self.AddSubPart( tsubpart1, num2, y + 100, 155, 35, 1);
      }
      tsubpart1 =  new TextButtonPartClass("Import Units", 155, "Allows you to import the units placed on the map in another scenario. This will remove all your current units and assigned officers. This only works if you have loaded the neccessary officer and historical unit libraries that are used by the units you are trying to import. Also this will overwrite the hex ownership. ",  self.OwnBitmap, num2 + 190, y + 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.importId = self.AddSubPart( tsubpart1, num2 + 190, y + 100, 155, 35, 1);
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num1: i32 = self.SubPartID[index1];
            if (num1 == self.VPListId)
            {
              let mut index2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (index2 > -1)
              {
                self.game.EditObj.UnitSelected = index2;
                let mut x1: i32 = self.game.Data.UnitObj[index2].X;
                let mut y1: i32 = self.game.Data.UnitObj[index2].Y;
                self.game.SelectX = x1;
                self.game.SelectY = y1;
                self.game.HandyFunctionsObj.CenterOnXY(x1, y1, true);
                windowReturnClass.AddCommand(4, 12);
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.AddUnitModel)
            {
              self.game.EditObj.TempHisModelUnit = -1;
              self.game.EditObj.TempHisUnit = -1;
              self.game.EditObj.TempRandom = -1;
              Form3::new( self.formref).Initialize(self.game.Data, 100, self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Regime, 0, self.game);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.standing1)
            {
              let mut unitCounter1: i32 = self.game.Data.UnitCounter;
              for (let mut index3: i32 = 0; index3 <= unitCounter1; index3 += 1)
              {
                if (self.game.Data.UnitObj[index3].Historical == self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical & (self.game.Data.Round == 0 | self.game.Data.UnitObj[index3].Regime == self.game.Data.Turn))
                {
                  if (self.game.Data.UnitObj[index3].SODefendPercent == 0)
                    self.game.Data.UnitObj[index3].SODefendPercent = 25;
                  else if (self.game.Data.UnitObj[index3].SODefendPercent == 25)
                    self.game.Data.UnitObj[index3].SODefendPercent = 50;
                  else if (self.game.Data.UnitObj[index3].SODefendPercent == 50)
                    self.game.Data.UnitObj[index3].SODefendPercent = 75;
                  else
                    self.game.Data.UnitObj[index3].SODefendPercent = 0;
                }
              }
              let mut unitCounter2: i32 = self.game.Data.UnitCounter;
              for (let mut unr: i32 = 0; unr <= unitCounter2; unr += 1)
              {
                if (self.game.HandyFunctionsObj.IsUnitInHQChain(unr, self.game.EditObj.UnitSelected))
                  self.game.Data.UnitObj[unr].SODefendPercent = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SODefendPercent;
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.standing2)
            {
              if (self.game.Data.Round == 0 | self.game.Data.Turn == self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime && self.game.HandyFunctionsObj.HasUnitAirSF(self.game.EditObj.UnitSelected))
              {
                if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SOInterceptRdnStop == 25)
                  self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SOInterceptRdnStop = 100;
                else if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SOInterceptRdnStop == 50)
                  self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SOInterceptRdnStop = 25;
                else if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SOInterceptRdnStop == 75)
                  self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SOInterceptRdnStop = 50;
                else
                  self.game.Data.UnitObj[self.game.EditObj.UnitSelected].SOInterceptRdnStop = 75;
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.AddUnit)
            {
              self.game.EditObj.TempHisModelUnit = -1;
              self.game.EditObj.TempHisUnit = -1;
              self.game.EditObj.TempRandom = -1;
              Form3::new( self.formref).Initialize(self.game.Data, 101, 0, 0, self.game);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.SetCommander)
            {
              self.game.EditObj.TempHisModelUnit = -1;
              self.game.EditObj.TempHisUnit = -1;
              self.game.EditObj.TempRandom = -1;
              Form3::new( self.formref).Initialize(self.game.Data, 103, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime, 0, self.game);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.HqUnit)
            {
              self.game.EditObj.OrderUnit = self.game.EditObj.UnitSelected;
              Form3::new( self.formref).Initialize(self.game.Data, 82, self.game.EditObj.OrderUnit, tGame: self.game);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.importId)
            {
              self.Import();
              self.DoStuff();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.changeColor)
            {
              ColorDialog colorDialog = ColorDialog::new();
              colorDialog.Color = Color.FromArgb( byte.MaxValue, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Red, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Green, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Blue);
              if (colorDialog.ShowDialog() == DialogResult.OK)
              {
                UnitClass unitClass1 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected];
                color: Color = colorDialog.Color;
                let mut b1: i32 =  color.B;
                unitClass1.Blue = b1;
                UnitClass unitClass2 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected];
                color = colorDialog.Color;
                let mut g: i32 =  color.G;
                unitClass2.Green = g;
                self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Red =  colorDialog.Color.R;
              }
              self.DoStuff();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.MoveUnit)
            {
              self.game.EditObj.OrderType = 1;
              self.game.EditObj.TempCoordList = self.game.HandyFunctionsObj.MakeMovePrediction(self.game.EditObj.UnitSelected, self.game.SelectX, self.game.SelectY, self.game.EditObj.MapSelected, attackoptions: true, ismove: true);
              self.game.EditObj.TempCoordList.RemoveCoord(0);
              self.game.EditObj.OrderUnit = self.game.EditObj.UnitSelected;
              self.game.EditObj.TempCoordList.AddCoord(self.game.SelectX, self.game.SelectY, self.game.EditObj.MapSelected);
              windowReturnClass.AddCommand(4, 12);
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.cancelid)
            {
              switch (self.game.EditObj.OrderType)
              {
                case 1:
                case 48:
                  self.game.EditObj.OrderType = 0;
                  if (self.game.EditObj.TempCoordList.counter < 3)
                    self.game.EditObj.TempCoordList = CoordList::new();
                  self.game.EditObj.UnitSelected = self.game.EditObj.OrderUnit;
                  self.game.SelectX = self.game.Data.UnitObj[self.game.EditObj.OrderUnit].X;
                  self.game.SelectY = self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Y;
                  self.game.EditObj.MapSelected = self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Map;
                  windowReturnClass.AddCommand(4, 12);
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                case 3:
                  self.game.EditObj.OrderType = 0;
                  self.game.EditObj.TempCoordList = CoordList::new();
                  self.game.EditObj.TempCoordList.AddCoord(self.game.SelectX, self.game.SelectY, self.game.EditObj.MapSelected);
                  self.game.EditObj.TempCoordList.AddCoord(self.game.EditObj.OrderX, self.game.EditObj.OrderY, self.game.EditObj.OrderMap);
                  windowReturnClass.AddCommand(4, 12);
                  self.game.SelectX = self.game.EditObj.OrderX;
                  self.game.SelectY = self.game.EditObj.OrderY;
                  self.game.EditObj.UnitSelected = self.game.EditObj.OrderUnit;
                  if (self.game.Data.Round == 0)
                    self.game.Data.Turn = -1;
                  self.DoStuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                default:
                  continue;
              }
            }
            else if (num1 == self.ChangeCounterUnit)
            {
              let mut historical: i32 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical;
              if (historical > -1)
              {
                str1: String = Interaction.InputBox("Give new name for unit", "Shadow Empire : Planetary Conquest", self.game.Data.HistoricalUnitObj[historical].Name.ToString());
                str2: String = Interaction.InputBox("Give new short name of unit", "Shadow Empire : Planetary Conquest", self.game.Data.HistoricalUnitObj[historical].CounterString);
                if (str1.Length > 0)
                  self.game.Data.HistoricalUnitObj[historical].Name = str1;
                self.game.Data.HistoricalUnitObj[historical].CounterString = str2;
                if (Operators.CompareString(Conversion.Val(str2).ToString(), str2, false) == 0)
                {
                  double a = Conversion.Val(str2);
                  if (self.game.Data.HistoricalUnitObj[historical].ModelMaster > -1 && a >  self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitObj[historical].ModelMaster].NameCounter)
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitObj[historical].ModelMaster].NameCounter =  Math.Round(a);
                }
                let mut unitCounter: i32 = self.game.Data.UnitCounter;
                for (let mut index4: i32 = 0; index4 <= unitCounter; index4 += 1)
                {
                  if (self.game.Data.UnitObj[index4].Historical == historical)
                    self.game.Data.UnitObj[index4].Name = self.game.Data.HistoricalUnitObj[historical].Name;
                }
                windowReturnClass.AddCommand(4, 12);
                self.game.SelectX = self.game.EditObj.OrderX;
                self.game.SelectY = self.game.EditObj.OrderY;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              let mut num2: i32 =  Interaction.MsgBox( "Unit is not set to a historical unit.");
            }
            else if (num1 == self.RemoveUnit)
            {
              if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].CommanderName.Length > 0)
              {
                LibIdClass libIdClass = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].OffLibId.Clone();
                self.game.Data.AddHistoricalUnit();
                self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].TempRegime = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].TempRegime;
                self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].People = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].People;
                self.game.ProcessingObj.SwapOfficer(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical, self.game.Data.HistoricalUnitCounter, self.game.EditObj.UnitSelected);
                self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].OffLibId = LibIdClass::new();
                self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].LibId = libIdClass;
                self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].Pool = true;
              }
              if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical].LibId.libSlot == -1)
              {
                self.game.Data.RemoveHistoricalUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical);
                for (let mut unitCounter: i32 = self.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
                {
                  if (self.game.Data.UnitObj[unitCounter].PreDef == -1 & self.game.Data.UnitObj[unitCounter].Historical == -1)
                  {
                    data: DataClass = self.game.Data;
                    let mut nr: i32 = unitCounter;
                    let mut gameClass: GameClass = (GameClass) null;
                     let mut local: GameClass =  gameClass;
                    data.RemoveUnit(nr,  local);
                  }
                }
              }
              else
              {
                let mut historical: i32 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical;
                for (let mut unitCounter: i32 = self.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
                {
                  if (self.game.Data.UnitObj[unitCounter].Historical == historical)
                  {
                    self.game.Data.UnitObj[unitCounter].Historical = -1;
                    data: DataClass = self.game.Data;
                    let mut nr: i32 = unitCounter;
                    let mut gameClass: GameClass = (GameClass) null;
                     let mut local: GameClass =  gameClass;
                    data.RemoveUnit(nr,  local);
                  }
                }
              }
              self.game.EditObj.UnitSelected = -1;
              windowReturnClass.AddCommand(4, 12);
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub fn Import()
    {
      str: String = "";
      path: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a scenario to import units/assigned officers from...", self.game.AppPath + self.game.ModScenarioDir, false);
      if (!File.Exists(path))
        return;
      let mut num1: i32 =  Interaction.MsgBox( "Ok hold on... this can take some time", Title: ( "Shadow Empire : Planetary Conquest"));
      self.game.FormRef.Cursor = Cursors.WaitCursor;
      self.game.EditObj.TempFileName = path;
      tempFileName: String = self.game.EditObj.TempFileName;
      self.game.HandyFunctionsObj.Unzip(tempFileName);
      dataClass1: DataClass = new DataClass(DontLoadGraphics: true);
      dataClass2: DataClass = DataClass.deserialize(tempFileName);
      self.game.HandyFunctionsObj.ZipFile(tempFileName);
      bool[] flagArray = new bool[dataClass2.UnitCounter + 1];
      if (dataClass2.MapObj[0].MapWidth > self.game.Data.MapObj[0].MapWidth | dataClass2.MapObj[0].MapHeight > self.game.Data.MapObj[0].MapHeight)
        str = "Map of import is larger than your current map.";
      if (str.Length == 0)
      {
        let mut unitCounter: i32 = dataClass2.UnitCounter;
        for (let mut index1: i32 = 0; index1 <= unitCounter; index1 += 1)
        {
          if (dataClass2.UnitObj[index1].PreDef == -1)
          {
            let mut historical: i32 = dataClass2.UnitObj[index1].Historical;
            if (dataClass2.HistoricalUnitObj[historical].CommanderName.Length > 0)
            {
              let mut libSlot: i32 = dataClass2.HistoricalUnitObj[historical].OffLibId.libSlot;
              if (libSlot > -1)
              {
                bool flag = false;
                if (self.game.Data.FindLibrary(dataClass2.LibraryObj[libSlot].name) > -1)
                {
                  let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
                  for (let mut index2: i32 = 0; index2 <= historicalUnitCounter; index2 += 1)
                  {
                    if (self.game.Data.HistoricalUnitObj[index2].LibId.id == dataClass2.HistoricalUnitObj[historical].OffLibId.id)
                      flag = true;
                    if (self.game.Data.HistoricalUnitObj[index2].OffLibId.id == dataClass2.HistoricalUnitObj[historical].OffLibId.id)
                      flag = true;
                  }
                  if (!flag)
                    str = str + "Current scenario's library: " + dataClass2.LibraryObj[libSlot].name + " is missing definition for officer: " + dataClass2.HistoricalUnitObj[historical].CommanderName;
                }
                else
                  str = str + "Current scenario missing library: " + dataClass2.LibraryObj[libSlot].name;
              }
              else
                str = str + dataClass2.HistoricalUnitObj[historical].CommanderName + "";
            }
            if (dataClass2.HistoricalUnitObj[historical].LibId.libSlot > -1)
            {
              let mut libSlot: i32 = dataClass2.HistoricalUnitObj[historical].LibId.libSlot;
              if (libSlot > -1)
              {
                bool flag = false;
                if (self.game.Data.FindLibrary(dataClass2.LibraryObj[libSlot].name) > -1)
                {
                  let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
                  for (let mut index3: i32 = 0; index3 <= historicalUnitCounter; index3 += 1)
                  {
                    if (self.game.Data.HistoricalUnitObj[index3].LibId.id == dataClass2.HistoricalUnitObj[historical].LibId.id)
                      flag = true;
                  }
                  if (!flag)
                    str = str + "Current scenario's library: " + dataClass2.LibraryObj[libSlot].name + " is missing definition for hist.unit: " + dataClass2.HistoricalUnitObj[historical].Name;
                }
                else
                  str = str + "Current scenario missing library: " + dataClass2.LibraryObj[libSlot].name;
              }
              else
                str = str + "Current scenario missing library: " + dataClass2.LibraryObj[libSlot].name;
            }
            else if (dataClass2.HistoricalUnitObj[historical].ModelMaster > -1)
            {
              let mut modelMaster: i32 = dataClass2.HistoricalUnitObj[historical].ModelMaster;
              if (modelMaster > -1)
              {
                if (dataClass2.HistoricalUnitObj[modelMaster].LibId.libSlot > -1)
                {
                  let mut libSlot: i32 = dataClass2.HistoricalUnitObj[modelMaster].LibId.libSlot;
                  if (libSlot > -1)
                  {
                    bool flag = false;
                    if (self.game.Data.FindLibrary(dataClass2.LibraryObj[libSlot].name) > -1)
                    {
                      let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
                      for (let mut index4: i32 = 0; index4 <= historicalUnitCounter; index4 += 1)
                      {
                        if (self.game.Data.HistoricalUnitObj[index4].LibId.id == dataClass2.HistoricalUnitObj[modelMaster].LibId.id)
                          flag = true;
                      }
                      if (!flag)
                        str = str + "Current scenario's library: " + dataClass2.LibraryObj[libSlot].name + " is missing definition for hist.unit model: " + dataClass2.HistoricalUnitObj[modelMaster].Name;
                    }
                    else
                      str = str + "Current scenario missing library: " + dataClass2.LibraryObj[libSlot].name;
                  }
                  else
                    str = str + "Current scenario missing library: " + dataClass2.LibraryObj[libSlot].name;
                }
                else
                  str = str + "Unit " + dataClass2.UnitObj[index1].Name + " using model that does not use a library.";
              }
              else
                str = str + "Unit " + dataClass2.UnitObj[index1].Name + " without library and without model.";
            }
          }
          if (str.Length > 0)
            break;
        }
      }
      if (str.Length == 0)
      {
        let mut mapWidth: i32 = dataClass2.MapObj[0].MapWidth;
        for (let mut index5: i32 = 0; index5 <= mapWidth; index5 += 1)
        {
          let mut mapHeight: i32 = dataClass2.MapObj[0].MapHeight;
          for (let mut index6: i32 = 0; index6 <= mapHeight; index6 += 1)
          {
            let mut num2: i32 = dataClass2.MapObj[0].HexObj[index5, index6].Regime <= -1 ? -1 : self.game.Data.FindRegimeFromSameLib( dataClass2.RegimeObj[dataClass2.MapObj[0].HexObj[index5, index6].Regime], "");
            self.game.Data.MapObj[0].HexObj[index5, index6].Regime = num2;
          }
        }
      }
      if (str.Length == 0)
      {
        for (let mut unitCounter: i32 = self.game.Data.UnitCounter; unitCounter >= 0; unitCounter += -1)
        {
          if (self.game.Data.UnitObj[unitCounter].PreDef == -1)
          {
            if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[unitCounter].Historical].OffLibId.libSlot > -1)
            {
              let mut tempRegime: i32 = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[unitCounter].Historical].TempRegime;
              LibIdClass libIdClass = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[unitCounter].Historical].OffLibId.Clone();
              self.game.Data.AddHistoricalUnit();
              self.game.ProcessingObj.SwapOfficer(self.game.Data.UnitObj[unitCounter].Regime, self.game.Data.UnitObj[unitCounter].Historical, self.game.Data.HistoricalUnitCounter, unitCounter);
              self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[unitCounter].Historical].OffLibId = LibIdClass::new();
              self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].LibId = libIdClass;
              self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].Pool = true;
              self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[unitCounter].Historical].TempRegime = tempRegime;
            }
            data: DataClass = self.game.Data;
            let mut nr: i32 = unitCounter;
            let mut gameClass: GameClass = (GameClass) null;
             let mut local: GameClass =  gameClass;
            data.RemoveUnit(nr,  local);
          }
        }
        for (let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter; historicalUnitCounter >= 0; historicalUnitCounter += -1)
        {
          if (self.game.Data.HistoricalUnitObj[historicalUnitCounter].LibId.libSlot == -1)
            self.game.Data.RemoveHistoricalUnit(historicalUnitCounter);
        }
      }
      if (str.Length == 0)
      {
        let mut unitCounter1: i32 = dataClass2.UnitCounter;
        for (let mut index7: i32 = 0; index7 <= unitCounter1; index7 += 1)
        {
          if (dataClass2.UnitObj[index7].PreDef == -1 & !flagArray[index7])
          {
            let mut historical: i32 = dataClass2.UnitObj[index7].Historical;
            if (dataClass2.HistoricalUnitObj[historical].LibId.libSlot > -1)
            {
              self.game.Data.FindRegimeFromSameLib( dataClass2.RegimeObj[dataClass2.HistoricalUnitObj[historical].TempRegime], dataClass2.LibraryObj[dataClass2.HistoricalUnitObj[historical].LibId.libSlot].name);
              let mut historicalFromSameLib: i32 = self.game.Data.FindHistoricalFromSameLib( dataClass2.HistoricalUnitObj[historical], dataClass2.LibraryObj[dataClass2.HistoricalUnitObj[historical].LibId.libSlot].name);
              self.game.EventRelatedObj.ExecAddHistoricalUnit(dataClass2.UnitObj[index7].X, dataClass2.UnitObj[index7].Y, self.game.Data.HistoricalUnitObj[historicalFromSameLib].ID, 0, "");
              let mut unitCounter2: i32 = dataClass2.UnitCounter;
              for (let mut index8: i32 = 0; index8 <= unitCounter2; index8 += 1)
              {
                if (dataClass2.UnitObj[index8].Historical == dataClass2.UnitObj[index7].Historical)
                {
                  flagArray[index8] = true;
                  for (let mut unitCounter3: i32 = self.game.Data.MapObj[0].HexObj[dataClass2.UnitObj[index7].X, dataClass2.UnitObj[index7].Y].UnitCounter; unitCounter3 >= 0; unitCounter3 += -1)
                  {
                    let mut unit: i32 = self.game.Data.MapObj[0].HexObj[dataClass2.UnitObj[index7].X, dataClass2.UnitObj[index7].Y].UnitList[unitCounter3];
                    if (self.game.Data.UnitObj[unit].Historical == historicalFromSameLib & self.game.Data.UnitObj[unit].HistoricalSubPart == dataClass2.UnitObj[index8].HistoricalSubPart)
                    {
                      self.game.Data.MapObj[0].HexObj[self.game.Data.UnitObj[unit].X, self.game.Data.UnitObj[unit].Y].RemoveUnitFromList(unit);
                      self.game.Data.MapObj[0].HexObj[dataClass2.UnitObj[index8].X, dataClass2.UnitObj[index8].Y].AddUnitToList(unit);
                    }
                  }
                }
              }
            }
            else
            {
              let mut regimeFromSameLib: i32 = self.game.Data.FindRegimeFromSameLib( dataClass2.RegimeObj[dataClass2.HistoricalUnitObj[historical].TempRegime], "");
              let mut modelMaster: i32 = dataClass2.HistoricalUnitObj[historical].ModelMaster;
              let mut historicalFromSameLib: i32 = self.game.Data.FindHistoricalFromSameLib( dataClass2.HistoricalUnitObj[modelMaster], dataClass2.LibraryObj[dataClass2.HistoricalUnitObj[modelMaster].LibId.libSlot].name);
              self.game.ProcessingObj.AddNewUnitBasedOnHistorical(dataClass2.UnitObj[index7].X, dataClass2.UnitObj[index7].Y, 0, regimeFromSameLib, historicalFromSameLib, freePPnoUnit: true, populateUnit: true);
              let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
              let mut unitCounter4: i32 = dataClass2.UnitCounter;
              for (let mut index9: i32 = 0; index9 <= unitCounter4; index9 += 1)
              {
                if (dataClass2.UnitObj[index9].Historical == dataClass2.UnitObj[index7].Historical)
                {
                  flagArray[index9] = true;
                  for (let mut unitCounter5: i32 = self.game.Data.MapObj[0].HexObj[dataClass2.UnitObj[index7].X, dataClass2.UnitObj[index7].Y].UnitCounter; unitCounter5 >= 0; unitCounter5 += -1)
                  {
                    let mut unit: i32 = self.game.Data.MapObj[0].HexObj[dataClass2.UnitObj[index7].X, dataClass2.UnitObj[index7].Y].UnitList[unitCounter5];
                    if (self.game.Data.UnitObj[unit].Historical == historicalUnitCounter & self.game.Data.UnitObj[unit].HistoricalSubPart == dataClass2.UnitObj[index9].HistoricalSubPart)
                    {
                      self.game.Data.MapObj[0].HexObj[self.game.Data.UnitObj[unit].X, self.game.Data.UnitObj[unit].Y].RemoveUnitFromList(unit);
                      self.game.Data.MapObj[0].HexObj[dataClass2.UnitObj[index9].X, dataClass2.UnitObj[index9].Y].AddUnitToList(unit);
                      self.game.Data.UnitObj[unit].X = dataClass2.UnitObj[index9].X;
                      self.game.Data.UnitObj[unit].Y = dataClass2.UnitObj[index9].Y;
                      self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[unit].Historical].Counter = dataClass2.HistoricalUnitObj[dataClass2.UnitObj[index9].Historical].Counter;
                      self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[unit].Historical].CounterString = dataClass2.HistoricalUnitObj[dataClass2.UnitObj[index9].Historical].CounterString;
                      break;
                    }
                  }
                }
              }
            }
            if (dataClass2.HistoricalUnitObj[historical].OffLibId.libSlot > -1)
            {
              let mut historicalBySameLib: i32 = self.game.Data.FindOffHistoricalBySameLib( dataClass2.HistoricalUnitObj[historical], dataClass2.LibraryObj[dataClass2.HistoricalUnitObj[historical].OffLibId.libSlot].name);
              LibIdClass libIdClass = self.game.Data.HistoricalUnitObj[historicalBySameLib].LibId.Clone();
              self.game.ProcessingObj.SwapOfficer(self.game.Data.UnitObj[self.game.Data.UnitCounter].Regime, self.game.Data.UnitObj[self.game.Data.UnitCounter].Historical, historicalBySameLib, self.game.Data.UnitCounter);
              self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.Data.UnitCounter].Historical].OffLibId = libIdClass.Clone();
            }
          }
        }
      }
      if (str.Length == 0)
      {
        let mut historicalUnitCounter1: i32 = self.game.Data.HistoricalUnitCounter;
        for (let mut index10: i32 = 0; index10 <= historicalUnitCounter1; index10 += 1)
        {
          if (self.game.Data.HistoricalUnitObj[index10].Model)
          {
            let mut historicalUnitCounter2: i32 = dataClass2.HistoricalUnitCounter;
            for (let mut index11: i32 = 0; index11 <= historicalUnitCounter2; index11 += 1)
            {
              if (dataClass2.HistoricalUnitObj[index11].Model && Operators.CompareString(self.game.Data.LibraryObj[self.game.Data.HistoricalUnitObj[index10].LibId.libSlot].name, dataClass2.LibraryObj[dataClass2.HistoricalUnitObj[index11].LibId.libSlot].name, false) == 0 && self.game.Data.HistoricalUnitObj[index10].LibId.id == dataClass2.HistoricalUnitObj[index11].LibId.id)
                self.game.Data.HistoricalUnitObj[index10].NameCounter = dataClass2.HistoricalUnitObj[index11].NameCounter;
            }
          }
        }
      }
      dataClass1 = (DataClass) null;
      self.game.EditObj.UnitSelected = -1;
      self.game.EditObj.OldUnit = -1;
      self.game.FormRef.Cursor = Cursors.Default;
      if (str.Length > 0)
      {
        let mut num3: i32 =  Interaction.MsgBox( ("ERROR IN IMPORT: " + str), Title: ( "Shadow Empire : Planetary Conquest"));
      }
      else
      {
        let mut num4: i32 =  Interaction.MsgBox( "Import completed succesfully", Title: ( "Shadow Empire : Planetary Conquest"));
      }
    }
  }
}
