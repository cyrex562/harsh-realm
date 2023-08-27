// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleEditDashboardWindowClass
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
  pub class SimpleEditDashboardWindowClass : WindowClass
  {
     listId: i32;
     ListClass listObj;
     loadMapId: i32;
     exportMapId: i32;
     setdateid: i32;
     exportid: i32;
     setdescriptid: i32;
     setnameid: i32;
     setdesignid: i32;
     loadMapIdB: i32;
     saveId: i32;
     textId: i32;
     text2id: i32;
     text3id: i32;
     detailnr: i32;
     currentStep: i32;

    pub SimpleEditDashboardWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 50, 9, tDoBorders: 1, tHeaderString: "Dashboard")
    {
      self.detailnr = -1;
      self.DoStuff();
    }

    pub fn PopUpRefresh() => self.DoStuff();

    pub fn DoStuff()
    {
      let mut val1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      if (self.listId > 0)
        self.RemoveSubPart(self.listId);
      if (self.loadMapId > 0)
        self.RemoveSubPart(self.loadMapId);
      if (self.exportMapId > 0)
        self.RemoveSubPart(self.exportMapId);
      if (self.setdateid > 0)
        self.RemoveSubPart(self.setdateid);
      if (self.loadMapIdB > 0)
        self.RemoveSubPart(self.loadMapIdB);
      if (self.textId > 0)
        self.RemoveSubPart(self.textId);
      if (self.text2id > 0)
        self.RemoveSubPart(self.text2id);
      if (self.text3id > 0)
        self.RemoveSubPart(self.text3id);
      if (self.saveId > 0)
        self.RemoveSubPart(self.saveId);
      if (self.setnameid > 0)
        self.RemoveSubPart(self.setnameid);
      if (self.setdescriptid > 0)
        self.RemoveSubPart(self.setdescriptid);
      if (self.setdesignid > 0)
        self.RemoveSubPart(self.setdesignid);
      if (self.exportid > 0)
        self.RemoveSubPart(self.exportid);
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      objgraphics.SmoothingMode = SmoothingMode.AntiAlias;
      objgraphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      objgraphics.TextContrast = 1;
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight - 50, -1);
      self.listObj = ListClass::new();
      self.currentStep = 0;
      self.listObj.add("CHECKLIST ITEM", 0, "IS SET?", "DETAILS");
      if (Information.IsNothing( self.game.Data.RuleSetName))
        self.game.Data.RuleSetName = "";
      ruleSetName: String = self.game.Data.RuleSetName;
      str: String = self.game.Data.scenarioVersionMaster.Length <= 0 ? ruleSetName + " (base version)" : ruleSetName + " (version " + self.game.Data.scenarioVersionMaster + ")";
      if (self.game.Data.RuleSetName.Length > 0)
      {
        self.listObj.add("Ruleset and Master", 1, "set", str);
        if (self.currentStep == 0)
          self.currentStep = 1;
      }
      else
        self.listObj.add("Ruleset and Master", 1, "not set", str);
      if (self.game.Data.MapObj[0].MapWidth == 0 | self.game.Data.MapObj[0].MapHeight == 0)
      {
        self.listObj.add("Map", 2, "not set", "Map:" + self.game.Data.MapObj[0].MapWidth.ToString() + "x" + self.game.Data.MapObj[0].MapHeight.ToString());
      }
      else
      {
        self.listObj.add("Map", 2, "set", "Map:" + self.game.Data.MapObj[0].MapWidth.ToString() + "x" + self.game.Data.MapObj[0].MapHeight.ToString());
        if (self.currentStep == 1)
          self.currentStep = 2;
      }
      let mut num1: i32 = 0;
      let mut sfTypeCounter: i32 = self.game.Data.SFTypeCounter;
      for (let mut index: i32 = 0; index <= sfTypeCounter; index += 1)
      {
        if (!self.game.Data.SFTypeObj[index].DontShowInList)
          num1 += 1;
      }
      if (num1 > 0)
      {
        self.listObj.add("Equipment&Troops Library", 3, "set", "SFTypes:" + num1.ToString());
        if (self.currentStep == 2)
          self.currentStep = 3;
      }
      else
        self.listObj.add("Equipment&Troops Library", 3, "not set", "SFTypes:" + num1.ToString());
      if (self.game.Data.HistoricalUnitCounter > -1)
      {
        self.listObj.add("Historical Library", 4, "set", "HisUnits:" + (self.game.Data.HistoricalUnitCounter + 1).ToString());
        if (self.currentStep == 3)
          self.currentStep = 4;
      }
      else
        self.listObj.add("Historical Library", 4, "not set", "HisUnits:" + (self.game.Data.HistoricalUnitCounter + 1).ToString());
      if (self.game.Data.StartData.Year > 1 & self.game.Data.AlternateRound > 0)
      {
        self.listObj.add("Start Date & round length", 5, "set", self.game.Data.StartData.Day.ToString() + "/" + self.game.Data.StartData.Month.ToString() + "/" + self.game.Data.StartData.Year.ToString() + " +d" + self.game.Data.AlternateRound.ToString());
        if (self.currentStep == 4)
          self.currentStep = 5;
      }
      else if (self.game.Data.StartData.Year > 1 & self.game.Data.AlternateRound2 > 0)
      {
        self.listObj.add("Start Date & round length", 5, "set", self.game.Data.StartData.Day.ToString() + "/" + self.game.Data.StartData.Month.ToString() + "/" + self.game.Data.StartData.Year.ToString() + " " + self.game.Data.StartData.Hour.ToString() + ":00 +h" + self.game.Data.AlternateRound2.ToString());
        if (self.currentStep == 4)
          self.currentStep = 5;
      }
      else
        self.listObj.add("Start Date & round length", 5, "not set");
      if (Operators.CompareString(self.game.Data.Name, "New Scenario", false) != 0 & Operators.CompareString(self.game.Data.Designer, "", false) != 0 & Operators.CompareString(self.game.Data.Description, "This is a blank scenario.", false) != 0)
      {
        self.listObj.add("Scenario Title,Descr,Designer", 6, "set", self.game.Data.Name);
        if (self.currentStep == 5)
          self.currentStep = 6;
      }
      else
        self.listObj.add("Scenario Title,Descr,Designer", 6, "not set");
      let mut num2: i32 = 0;
      let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
      for (let mut index1: i32 = 0; index1 <= regimeCounter; index1 += 1)
      {
        bool flag = false;
        let mut unitCounter: i32 = self.game.Data.UnitCounter;
        for (let mut index2: i32 = 0; index2 <= unitCounter; index2 += 1)
        {
          if (self.game.Data.UnitObj[index2].IsHQ & self.game.Data.UnitObj[index2].PreDef == -1 & self.game.Data.UnitObj[index2].X > -1)
            flag = true;
        }
        if (flag)
          num2 += 1;
      }
      if (num2 >= 2)
      {
        self.listObj.add("2 regimes or more with HQs on map ", 7, "set", num2.ToString());
        if (self.currentStep == 6)
          self.currentStep = 7;
      }
      else
        self.listObj.add("2 regimes or more with HQs on map", 7, "not set", num2.ToString());
      let mut num3: i32 = 0;
      let mut num4: i32 = 0;
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index3: i32 = 0; index3 <= mapWidth; index3 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index4: i32 = 0; index4 <= mapHeight; index4 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[index3, index4].VP > 0)
          {
            num3 += 1;
            num4 += self.game.Data.MapObj[0].HexObj[index3, index4].VP;
          }
        }
      }
      if (num3 >= 1)
      {
        self.listObj.add("Victory Points on map ", 8, "set", num3.ToString() + "(" + num4.ToString() + ")");
        if (self.currentStep == 7)
          self.currentStep = 8;
      }
      else
        self.listObj.add("Victory Points on map", 8, "not set", num3.ToString());
      let mut index5: i32 = -1;
      let mut eventCounter1: i32 = self.game.Data.EventCounter;
      for (let mut index6: i32 = 0; index6 <= eventCounter1; index6 += 1)
      {
        let mut commandCounter: i32 = self.game.Data.EventObj[index6].CommandCounter;
        for (let mut index7: i32 = 0; index7 <= commandCounter; index7 += 1)
        {
          if (self.game.Data.EventObj[index6].CommandList[index7].type == 3 && Conversions.ToDouble(self.game.Data.EventObj[index6].CommandList[index7].Data[0, 1]) == 30.0)
            index5 = index6;
        }
      }
      if (index5 > -1)
      {
        self.listObj.add("Supply event present", 9, "set", "Event: '" + self.game.Data.EventObj[index5].Name + "'");
        if (self.currentStep == 8)
          self.currentStep = 9;
      }
      else
        self.listObj.add("Supply event present", 9, "not set");
      let mut index8: i32 = -1;
      let mut eventCounter2: i32 = self.game.Data.EventCounter;
      for (let mut index9: i32 = 0; index9 <= eventCounter2; index9 += 1)
      {
        let mut commandCounter: i32 = self.game.Data.EventObj[index9].CommandCounter;
        for (let mut index10: i32 = 0; index10 <= commandCounter; index10 += 1)
        {
          if (self.game.Data.EventObj[index9].CommandList[index10].type == 3 && Conversions.ToDouble(self.game.Data.EventObj[index9].CommandList[index10].Data[0, 1]) == 4.0)
            index8 = index9;
        }
      }
      if (index8 > -1)
      {
        self.listObj.add("Victory event present", 10, "set", "Event: '" + self.game.Data.EventObj[index8].Name + "'");
        if (self.currentStep == 9)
          self.currentStep = 10;
      }
      else
        self.listObj.add("Victory event present", 10, "not set");
      if (0 > -1)
      {
        self.listObj.add("Distribute ready?", 11, "Yes");
        if (self.currentStep == 10)
          self.currentStep = 10;
      }
      else
        self.listObj.add("Distribute ready?", 11, "Don't think so");
      if (self.detailnr == -1)
        self.detailnr = self.currentStep + 1;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(self.listObj, 18, 500 + Math.Max(0, val1 - 50), self.detailnr, self.game, true, "Checklist", false, tShowPair: true, tValueWidth: ( Math.Round(260.0 +  val1 * 0.8)), tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (10 + Math.Min(val1, 50)), bby: 72, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
      self.listId = self.AddSubPart( tsubpart, 10 + Math.Min(val1, 50), 72, 500 + Math.Max(0, val1 - 50), 504, 0);
      tsubpart =  new TextButtonPartClass("Save Scenario", 180, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 10), bby: 588, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.saveId = self.AddSubPart( tsubpart, val1 + 10, 588, 180, 35, 1);
      DrawMod.DrawTextColouredMarc( objgraphics, "Saving also refreshes units rdn,ap,entr, etc.. Save before you play.", self.game.MarcFont4, val1 + 10, 628, Color.White);
      let mut num5: i32 = self.detailnr - 1;
      if (num5 == 0)
      {
        str = "Changing the Ruleset is not allowed. But the Master will be reloaded every time this scenario is loaded. Or can be manually reloaded through the 'debug tab'.";
        DrawMod.DrawTextColouredMarc( objgraphics, "Step 0: Ruleset", self.game.MarcFont1, val1 + 580, 70, Color.White);
        tsubpart =  new TextAreaClass2(self.game, 450, 4, self.game.MarcFont3, str, 27,  self.OwnBitmap, val1 + 570, 100, true, true);
        self.textId = self.AddSubPart( tsubpart, val1 + 570, 100, 450, 108, 0);
      }
      if (num5 == 1)
      {
        if (self.currentStep > num5)
        {
          tText1: String = "You already have a Map loaded. This step has been completed.";
          DrawMod.DrawTextColouredMarc( objgraphics, "Step 1: Load Map", self.game.MarcFont1, val1 + 580, 70, Color.White);
          tsubpart =  new TextAreaClass2(self.game, 450, 4, self.game.MarcFont3, tText1, 27,  self.OwnBitmap, val1 + 570, 100, true, true);
          self.textId = self.AddSubPart( tsubpart, val1 + 570, 100, 450, 108, 0);
          tsubpart =  new TextButtonPartClass("Reload Map", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 580), bby: 200, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.loadMapId = self.AddSubPart( tsubpart, val1 + 580, 240, 140, 35, 1);
          tText2: String = self.game.Data.MapName + ", version " + self.game.Data.MapVersion.ToString();
          DrawMod.DrawTextColouredMarc( objgraphics, "Map name and version", self.game.MarcFont1, val1 + 580, 320, Color.White);
          tsubpart =  new TextAreaClass2(self.game, 450, 2, self.game.MarcFont3, tText2, 27,  self.OwnBitmap, val1 + 570, 320, true, true);
          self.text2id = self.AddSubPart( tsubpart, val1 + 570, 320, 450, 54, 0);
          str = self.game.Data.MapDesigner;
          DrawMod.DrawTextColouredMarc( objgraphics, "Map designer", self.game.MarcFont1, val1 + 580, 420, Color.White);
          tsubpart =  new TextAreaClass2(self.game, 450, 2, self.game.MarcFont3, str, 27,  self.OwnBitmap, val1 + 570, 420, true, true);
          self.text3id = self.AddSubPart( tsubpart, val1 + 570, 420, 450, 54, 0);
        }
        else
        {
          str = "Please select a Map compatible with the chosen ruleset.";
          DrawMod.DrawTextColouredMarc( objgraphics, "Step 1: Load Map", self.game.MarcFont1, val1 + 580, 70, Color.White);
          tsubpart =  new TextAreaClass2(self.game, 450, 4, self.game.MarcFont3, str, 27,  self.OwnBitmap, val1 + 570, 100, true, true);
          self.textId = self.AddSubPart( tsubpart, val1 + 570, 100, 450, 108, 0);
          tsubpart =  new TextButtonPartClass("Load Map", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 580), bby: 200, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.loadMapId = self.AddSubPart( tsubpart, val1 + 580, 240, 140, 35, 1);
        }
      }
      if (num5 == 2)
      {
        str = self.currentStep <= num5 ? "There is no Equipment&Troops Library loaded at the moment. Please go the library section and import at least one Equipment&Troops Library compatible with the chosen ruleset." : "You have already loaded one or more Equipment&Troops Libraries, but you can always load more or remove some.";
        DrawMod.DrawTextColouredMarc( objgraphics, "Step 2: Load Equipment", self.game.MarcFont1, val1 + 580, 70, Color.White);
        tsubpart =  new TextAreaClass2(self.game, 450, 4, self.game.MarcFont3, str, 27,  self.OwnBitmap, val1 + 570, 100, true, true);
        self.textId = self.AddSubPart( tsubpart, val1 + 570, 100, 450, 108, 0);
      }
      if (num5 == 3)
      {
        str = self.currentStep <= num5 ? "There is no Historical Library loaded at the moment. Please go the library section and import at least one Historical Library compatible with the chosen ruleset." : "You have already loaded one or more Historical Libraries, but you can always load more or remove some.";
        DrawMod.DrawTextColouredMarc( objgraphics, "Step 3: Load Historicals", self.game.MarcFont1, val1 + 580, 70, Color.White);
        tsubpart =  new TextAreaClass2(self.game, 450, 4, self.game.MarcFont3, str, 27,  self.OwnBitmap, val1 + 570, 100, true, true);
        self.textId = self.AddSubPart( tsubpart, val1 + 570, 100, 450, 108, 0);
      }
      if (num5 == 4)
      {
        if (self.currentStep > num5)
        {
          if (self.game.Data.AlternateRound > 0)
            str = "Current date for the first round is: '" + self.game.Data.StartData.ToString() + "'. And every round lasts " + self.game.Data.AlternateRound.ToString() + " days. Please feel free to change it.";
          else if (self.game.Data.AlternateRound2 > 0)
            str = "Current date for the first round is: '" + self.game.Data.StartData.ToString() + "'. And every round lasts " + self.game.Data.AlternateRound2.ToString() + " hours. Please feel free to change it.";
        }
        else
          str = "Pease specify a date for the first round.";
        tsubpart =  new TextButtonPartClass("Set Date", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 580), bby: 280, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.setdateid = self.AddSubPart( tsubpart, val1 + 580, 280, 140, 35, 1);
        DrawMod.DrawTextColouredMarc( objgraphics, "Step 4: Set start date & round length", self.game.MarcFont1, val1 + 580, 70, Color.White);
        tsubpart =  new TextAreaClass2(self.game, 450, 4, self.game.MarcFont3, str, 27,  self.OwnBitmap, val1 + 570, 100, true, true);
        self.textId = self.AddSubPart( tsubpart, val1 + 570, 100, 450, 108, 0);
      }
      if (num5 == 5)
      {
        description: String = self.game.Data.Description;
        tsubpart =  new TextButtonPartClass("Change Name", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 580), bby: 140, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.setnameid = self.AddSubPart( tsubpart, val1 + 580, 140, 140, 35, 1);
        tsubpart =  new TextButtonPartClass("Change Descript", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 580), bby: 580, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.setdescriptid = self.AddSubPart( tsubpart, val1 + 580, 580, 140, 35, 1);
        tsubpart =  new TextButtonPartClass("Change Designer", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 580), bby: 300, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.setdesignid = self.AddSubPart( tsubpart, val1 + 580, 300, 140, 35, 1);
        DrawMod.DrawTextColouredMarc( objgraphics, "Step 5a: Set scenario name", self.game.MarcFont1, val1 + 580, 70, Color.White);
        DrawMod.DrawTextColouredMarc( objgraphics, self.game.Data.Name, self.game.MarcFont3, val1 + 580, 110, Color.White);
        DrawMod.DrawTextColouredMarc( objgraphics, "Step 5b: Set scenario designer", self.game.MarcFont1, val1 + 580, 230, Color.White);
        if (self.game.Data.Designer.Length > 0)
          DrawMod.DrawTextColouredMarc( objgraphics, self.game.Data.Designer, self.game.MarcFont3, val1 + 580, 270, Color.White);
        else
          DrawMod.DrawTextColouredMarc( objgraphics, "-Nobody specified-", self.game.MarcFont3, val1 + 580, 270, Color.White);
        DrawMod.DrawTextColouredMarc( objgraphics, "Step 5c: Set scenario descript", self.game.MarcFont1, val1 + 580, 390, Color.White);
        tsubpart =  new TextAreaClass2(self.game, 450, 4, self.game.MarcFont4, description, 27,  self.OwnBitmap, val1 + 570, 410);
        self.textId = self.AddSubPart( tsubpart, val1 + 570, 410, 450, 108, 0);
      }
      if (num5 == 6)
      {
        tText: String = self.currentStep <= num5 ? "Make sure you have loaded a map with regimes defined if you are missing regimes. Make sure you have put some units on the map for each side otherwise. You need to have HQ for each side in order to receive replacement troops and supplies." : "You have at least 2 regimes with a HQ on the map. All set here. ";
        DrawMod.DrawTextColouredMarc( objgraphics, "Step 6: Minimal 2 regimes with a HQ on map", self.game.MarcFont1, val1 + 580, 70, Color.White);
        tsubpart =  new TextAreaClass2(self.game, 450, 4, self.game.MarcFont3, tText, 27,  self.OwnBitmap, val1 + 570, 100, true, true);
        self.textId = self.AddSubPart( tsubpart, val1 + 570, 100, 450, 108, 0);
      }
      if (num5 == 7)
      {
        tText: String = self.currentStep <= num5 ? "You need to put some victory points on some hexes on the map." : "You have at least 1 Victory Poset: i32. Make sure you have enough different ones defined to make the AI able to handle it self. Around 1 VP every 64 hexes would be around optimal. Keep in mind that the AI can have different behaviour based on the number of VP on a hex. ";
        DrawMod.DrawTextColouredMarc( objgraphics, "Step 7: Victory Points on the map", self.game.MarcFont1, val1 + 580, 70, Color.White);
        tsubpart =  new TextAreaClass2(self.game, 450, 8, self.game.MarcFont3, tText, 27,  self.OwnBitmap, val1 + 570, 100, true, true);
        self.textId = self.AddSubPart( tsubpart, val1 + 570, 100, 450, 216, 0);
      }
      if (num5 == 8)
      {
        tText: String = self.currentStep <= num5 ? "You need to load a library that includes a supply event. Without supply being delivered to your units this scenario will probably not function as intended." : "Altough a Supply event has been loaded you have to make sure you set the propper input for it in stringlists or libvars in order for it to function properly.";
        DrawMod.DrawTextColouredMarc( objgraphics, "Step 8: Supply event present", self.game.MarcFont1, val1 + 580, 70, Color.White);
        tsubpart =  new TextAreaClass2(self.game, 450, 4, self.game.MarcFont3, tText, 27,  self.OwnBitmap, val1 + 570, 100, true, true);
        self.textId = self.AddSubPart( tsubpart, val1 + 570, 100, 450, 108, 0);
      }
      if (num5 == 9)
      {
        tText: String = self.currentStep <= num5 ? "You need to load a library that includes a victory event. Without a victory event being set this scenario will probably be unable to end in victory for any player." : "Altough a Victory event has been loaded you have to make sure you set the propper input for it in stringlists or libvars in order for it to function properly.";
        DrawMod.DrawTextColouredMarc( objgraphics, "Step 9: Victory event present", self.game.MarcFont1, val1 + 580, 70, Color.White);
        tsubpart =  new TextAreaClass2(self.game, 450, 4, self.game.MarcFont3, tText, 27,  self.OwnBitmap, val1 + 570, 100, true, true);
        self.textId = self.AddSubPart( tsubpart, val1 + 570, 100, 450, 108, 0);
      }
      if (num5 != 10)
        return;
      tText3: String = (self.currentStep < num5 ? "There are still some steps that test negative. This might be intended, but my advice is to make sure all steps above are set before distributing." : "Your scenario seems to be ready for distribution. Make sure all the events run well and its play-tested before sharing with others.") + "\r\n" + "\r\n" + "1. Before exporting. Make sure all your custom graphics are in their own custom directories directly under the graphics/ directory and not in the graphics/community or graphics/communitymodgraphics or graphics/systemgraphics directories." + "\r\n" + "\r\n" + "2. Optional. You can add a few custom files like .pdf or .txt as you see fit.. A readme.txt file if included will be presented to the end-user when he imports the zip. Have your list of extra files ready. " + "\r\n" + "\r\n" + "3. Optional. Make a check if you have used any music or sounds that are not installed by default. If so you should have the name of your sound directory ready; which should be directly under graphics/communitysounds/ and should not have any further subdirectories." + "\r\n" + "\r\n" + "4. Ok press the export button below!";
      tsubpart =  new TextButtonPartClass("Export .dczip", 140, tBackbitmap: ( self.OwnBitmap), bbx: (val1 + 580), bby: 620, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.exportid = self.AddSubPart( tsubpart, val1 + 580, 620, 140, 35, 1);
      DrawMod.DrawTextColouredMarc( objgraphics, "Distributing your scenario", self.game.MarcFont1, val1 + 580, 70, Color.White);
      tsubpart =  new TextAreaClass2(self.game, 450, 24, self.game.MarcFont3, tText3, 19,  self.OwnBitmap, val1 + 570, 100, true, true);
      self.textId = self.AddSubPart( tsubpart, val1 + 570, 100, 450, 456, 0);
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num1: i32 = self.SubPartID[index];
            if (num1 == self.listId)
            {
              self.detailnr = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              if (self.detailnr < 1)
                self.detailnr = 1;
              self.DoStuff();
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.setnameid)
            {
              self.game.Data.Name = Interaction.InputBox("Give new scenario name", "Shadow Empire : Planetary Conquest");
              self.DoStuff();
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.setdescriptid)
            {
              Form2::new( self.formref).Initialize(self.game.Data, 2, -1);
              return windowReturnClass;
            }
            if (num1 == self.setdesignid)
            {
              self.game.Data.Designer = Interaction.InputBox("Give designer name", "Shadow Empire : Planetary Conquest");
              self.DoStuff();
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.setdateid)
            {
              if (Interaction.MsgBox( "Use the normal days system for rounds? (yes = use days, no = use hours)", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              {
                str1: String = Interaction.InputBox("Give new Day (1-31).", "Shadow Empire : Planetary Conquest");
                if (Conversions.ToInteger(str1) >= 1 & Conversions.ToInteger(str1) <= 31)
                {
                  let mut integer1: i32 = Conversions.ToInteger(str1);
                  str2: String = Interaction.InputBox("Give new Month (1-12).", "Shadow Empire : Planetary Conquest");
                  if (Conversions.ToInteger(str2) >= 1 & Conversions.ToInteger(str2) <= 12)
                  {
                    let mut integer2: i32 = Conversions.ToInteger(str2);
                    str3: String = Interaction.InputBox("Give new Year (2-9999).", "Shadow Empire : Planetary Conquest");
                    if (Conversions.ToInteger(str3) >= 2 & Conversions.ToInteger(str3) <= 9999)
                    {
                      let mut integer3: i32 = Conversions.ToInteger(str3);
                      str4: String = Interaction.InputBox("Give new round length in days (1-999).", "Shadow Empire : Planetary Conquest");
                      if (Conversions.ToInteger(str4) >= 1 & Conversions.ToInteger(str4) <= 9999)
                      {
                        let mut integer4: i32 = Conversions.ToInteger(str4);
                        self.game.Data.StartData = new DateTime(integer3, integer2, integer1);
                        self.game.Data.AlternateRound2 = -1;
                        self.game.Data.AlternateRound = integer4;
                      }
                      else
                      {
                        let mut num2: i32 =  Interaction.MsgBox( "Sorry. Invalid round length.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                    }
                    else
                    {
                      let mut num3: i32 =  Interaction.MsgBox( "Sorry. Invalid year.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                  }
                  else
                  {
                    let mut num4: i32 =  Interaction.MsgBox( "Sorry. Invalid month.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                }
                else
                {
                  let mut num5: i32 =  Interaction.MsgBox( "Sorry. Invalid day of the month.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
              }
              else
              {
                str5: String = Interaction.InputBox("Give new Day (1-31).", "Shadow Empire : Planetary Conquest");
                if (Conversions.ToInteger(str5) >= 1 & Conversions.ToInteger(str5) <= 31)
                {
                  let mut integer5: i32 = Conversions.ToInteger(str5);
                  str6: String = Interaction.InputBox("Give new Month (1-12).", "Shadow Empire : Planetary Conquest");
                  if (Conversions.ToInteger(str6) >= 1 & Conversions.ToInteger(str6) <= 12)
                  {
                    let mut integer6: i32 = Conversions.ToInteger(str6);
                    str7: String = Interaction.InputBox("Give new Year (2-9999).", "Shadow Empire : Planetary Conquest");
                    if (Conversions.ToInteger(str7) >= 2 & Conversions.ToInteger(str7) <= 9999)
                    {
                      let mut integer7: i32 = Conversions.ToInteger(str7);
                      str8: String = Interaction.InputBox("Give new Hour (1-24).", "Shadow Empire : Planetary Conquest");
                      if (Conversions.ToInteger(str8) >= 1 & Conversions.ToInteger(str8) <= 24)
                      {
                        let mut integer8: i32 = Conversions.ToInteger(str8);
                        str9: String = Interaction.InputBox("Give new round length in hours (1-12).", "Shadow Empire : Planetary Conquest");
                        if (Conversions.ToInteger(str9) >= 1 & Conversions.ToInteger(str9) <= 12)
                        {
                          let mut integer9: i32 = Conversions.ToInteger(str9);
                          self.game.Data.StartData = new DateTime(integer7, integer6, integer5, integer8, 0, 0);
                          self.game.Data.AlternateRound = -1;
                          self.game.Data.AlternateRound2 = integer9;
                        }
                        else
                        {
                          let mut num6: i32 =  Interaction.MsgBox( "Sorry. Invalid round length.", Title: ( "Shadow Empire : Planetary Conquest"));
                        }
                      }
                      else
                      {
                        let mut num7: i32 =  Interaction.MsgBox( "Sorry. Invalid hour.", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                    }
                    else
                    {
                      let mut num8: i32 =  Interaction.MsgBox( "Sorry. Invalid year.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                  }
                  else
                  {
                    let mut num9: i32 =  Interaction.MsgBox( "Sorry. Invalid month.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                }
                else
                {
                  let mut num10: i32 =  Interaction.MsgBox( "Sorry. Invalid day of the month.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.saveId)
            {
              tinitdir: String;
              if (self.game.Data.Round == 0)
              {
                tinitdir = self.game.AppPath + "scenarios\\";
                if (!Information.IsNothing( self.game.Data.ScenarioDir))
                {
                  if (self.game.Data.ScenarioDir.Length > 1)
                    tinitdir = tinitdir.Replace("scenarios", self.game.Data.ScenarioDir);
                  else if (self.game.ModScenarioDir.Length > 1)
                    tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
                }
                else if (self.game.ModScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              }
              else
                tinitdir = "savedgames";
              str: String = self.game.Data.Round != 0 ? self.game.HandyFunctionsObj.SaveSomething("SE1 Scenario file (*.se1)|*.se1", "Give save name...", tinitdir, false) : self.game.HandyFunctionsObj.SaveSomething("SE1 Scenario file (*.se1)|*.se1", "Give save name...", tinitdir, false);
              if (Strings.Len(str) >= 2)
              {
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.game.HandyFunctionsObj.SetAllReady(false);
                self.game.Data.serialize(str);
                self.game.HandyFunctionsObj.ZipFile(str);
                windowReturnClass.SetFlag(true);
                self.game.FormRef.Cursor = Cursors.Default;
                let mut num11: i32 =  Interaction.MsgBox( "Saved", Title: ( "Shadow Empire : Planetary Conquest"));
              }
            }
            else
            {
              if (num1 == -870624953)
              {
                str: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Map file(*.se1map)|*.se1map", "Pick a map to load...", self.game.AppPath + self.game.ModScenarioDir, false);
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                bool flag = File.Exists(str) && (!(self.game.Data.MapObj[0].MapWidth == 0 | self.game.Data.MapObj[0].MapHeight == 0) ? self.game.HandyFunctionsObj.LoadMap(str, true, true) : self.game.HandyFunctionsObj.LoadMap(str, true));
                self.game.FormRef.Cursor = Cursors.Default;
                if (flag)
                {
                  let mut num12: i32 =  Interaction.MsgBox( "Loaded map", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  let mut num13: i32 =  Interaction.MsgBox( "Error while loading map", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.detailnr = -1;
                self.DoStuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.loadMapId)
              {
                path: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Map file(*.se1map)|*.se1map", "Pick a map to load...", self.game.AppPath + self.game.ModScenarioDir, false);
                if (File.Exists(path))
                {
                  self.game.EditObj.TempFileName = path;
                  self.game.EditObj.PopupValue = 18;
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  windowReturnClass.AddCommand(5, 10);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut num14: i32 =  Interaction.MsgBox( "Could not find file", Title: ( "Shadow Empire : Planetary Conquest"));
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.exportid)
              {
                self.game.HandyFunctionsObj.ExportSimpleEditor();
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
