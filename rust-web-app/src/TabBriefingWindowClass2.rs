// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TabBriefingWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class TabBriefingWindowClass2 : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     ShowString: String;
     DateTime ShowTime;
     w: i32;
     h: i32;
     CurrentView: i32;

    pub TabBriefingWindowClass2(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect,
      Rectangle trect)
      : base( tGame, trect.Width, trect.Height, 8)
    {
      self.w = trect.Width;
      self.h = trect.Height;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.dostuff();
    }

    pub fn DoRefresh() => self.dostuff();

    pub fn dostuff()
    {
      self.ClearMouse();
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Rectangle trect = DrawMod.DrawBackTab(Graphics.FromImage((Image) self.OwnBitmap), self.w, self.h, "BRIEF", 1);
      self.AddMouse( trect, "CLOSE TAB", "Click here to close this tab. [ESC/F2]", 999);
      String1_1: String = self.game.Data.Description;
      if (self.game.Data.Product >= 6)
      {
        if (Strings.InStr(String1_1, "[tab]") <= 0)
          String1_1 = "[tab]Scenario description," + String1_1 + "[/tab]";
        str1: String = String1_1 + "[tab]Version info," + "Game version: " + "v " + Strings.Trim(Conversion.Str( Math.Floor(1.1))) + "." + Strings.Trim(Conversion.Str( 10)) + " " + Strings.Trim(".04b") + " Shadow Empire : Planetary Conquest. " + "\r\n" + "Scenario name: " + self.game.Data.Name + "\r\n";
        if (self.game.Data.scenarioVersion.Length >= 2)
          str1 = str1 + "Scenario version: " + self.game.Data.scenarioVersion + "\r\n";
        else if (self.game.Data.RegimeObj[self.game.Data.Turn].Version > 0)
        {
          str2: String = str1 + "Last turn game version: ";
          str3: String;
          if ((self.game.Data.RegimeObj[self.game.Data.Turn].Version - 314) % 100 >= 10)
            str3 = str2 + "v " + Strings.Trim(Conversion.Str( Math.Floor( (self.game.Data.RegimeObj[self.game.Data.Turn].Version - 314) / 100.0))) + "." + Strings.Trim(Conversion.Str( 10));
          else
            str3 = str2 + "v " + Strings.Trim(Conversion.Str( Math.Floor( (self.game.Data.RegimeObj[self.game.Data.Turn].Version - 314) / 100.0))) + ".0" + Strings.Trim(Conversion.Str( 10));
          if (!Information.IsNothing( self.game.Data.RegimeObj[self.game.Data.Turn].subVersion))
            str3 += self.game.Data.RegimeObj[self.game.Data.Turn].subVersion;
          str1 = str3 + "\r\n";
        }
        String1_1 = str1 + "Scenario master version: " + self.game.Data.scenarioVersionMaster + "\r\n" + "[/tab]";
      }
      str4: String = String1_1 + "[tab]Config," + "This game was started with the following variants configuration: \r\n\r\n";
      let mut num: i32 = 0;
      let mut index1: i32 = 0;
      do
      {
        if (self.game.Data.Variants[index1] > -1)
        {
          String1_2: String = self.game.Data.GameSlotName[self.game.Data.Variants[index1]];
          if (Strings.InStr(String1_2, ",") > 0)
            String1_2 = String1_2.Split(',')[0];
          str5: String = (self.game.Data.GameSlot[self.game.Data.Variants[index1]] <= 0 ? String1_2 + " is off." : String1_2 + " is ON.") + "\r\n";
          str4 += str5;
          num += 1;
        }
        index1 += 1;
      }
      while (index1 <= 11);
      if (num == 0)
        str4 += "No variants selected.";
      let mut index2: i32 = -1;
      let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
      for (let mut index3: i32 = 0; index3 <= regimeCounter; index3 += 1)
      {
        if (self.game.Data.RegimeObj[index3].AI & !self.game.Data.RegimeObj[index3].Sleep)
          index2 = index3;
      }
      if (index2 > -1)
      {
        str6: String = str4 + "\r\n" + "The AI is set to speed level ";
        str7: String = (self.game.Data.RegimeObj[index2].ProdBonus < 250 ? (self.game.Data.RegimeObj[index2].ProdBonus < 100 ? str6 + "FAST" : str6 + "NORMAL") : str6 + "SLOW") + ".\r\n" + "The AI difficulty is set to level ";
        str4 = (self.game.Data.Product != 6 ? (self.game.Data.RegimeObj[index2].AIHelpMove < 50 ? (self.game.Data.RegimeObj[index2].AIHelpMove < 40 ? (self.game.Data.RegimeObj[index2].AIHelpMove < 30 ? (self.game.Data.RegimeObj[index2].AIHelpMove < 20 ? str7 + "NORMAL" : str7 + "CHALLENGING") : str7 + "HARD") : str7 + "VERY HARD") : str7 + "SUPER HARD") : (self.game.Data.RegimeObj[index2].AIHelpCombat < 55 ? (self.game.Data.RegimeObj[index2].AIHelpCombat < 40 ? (self.game.Data.RegimeObj[index2].AIHelpCombat < 25 ? (self.game.Data.RegimeObj[index2].AIHelpCombat < 15 ? str7 + "NORMAL" : str7 + "CHALLENGING") : str7 + "HARD") : str7 + "VERY HARD") : str7 + "SUPER HARD")) + ".\r\n";
      }
      tText: String = str4 + "[/tab]";
      if (self.Info1Id == 0)
      {
        let mut tsubpart: SubPartClass =  new TextAreaClass2(self.game, self.w - 40, 16, self.game.MarcFont8, tText, 17,  self.OwnBitmap, 20, 0, tUseEncy: true);
        self.Info1Id = self.AddSubPart( tsubpart, 20, 0, self.w - 14, 306, 0);
      }
      else
        self.SubPartFlag[self.SubpartNr(self.Info1Id)] = true;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          return;
        }
      }
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
        {
          self.game.EditObj.TipButton = false;
          self.SubPartList[index].HandleToolTip(x - self.SubPartX[index], y - self.SubPartY[index]);
          if (self.game.EditObj.TipButton)
            break;
          if (self.SubPartID[index] != self.Info1Id)
            ;
        }
      }
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (nr == 40)
      {
        self.SubPartList[self.SubpartNr(self.Info1Id)].ShiftDown();
        self.SubPartFlag[self.SubpartNr(self.Info1Id)] = true;
        windowReturnClass.SetFlag(true);
      }
      if (nr == 38)
      {
        self.SubPartList[self.SubpartNr(self.Info1Id)].ShiftUp();
        self.SubPartFlag[self.SubpartNr(self.Info1Id)] = true;
        windowReturnClass.SetFlag(true);
      }
      if (nr == 37)
      {
        self.SubPartList[self.SubpartNr(self.Info1Id)].ShiftLeft();
        self.SubPartFlag[self.SubpartNr(self.Info1Id)] = true;
        windowReturnClass.SetFlag(true);
      }
      if (nr == 39)
      {
        self.SubPartList[self.SubpartNr(self.Info1Id)].ShiftRight();
        self.SubPartFlag[self.SubpartNr(self.Info1Id)] = true;
        windowReturnClass.SetFlag(true);
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (self.MouseData[index] > 0 && x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height && self.MouseData[index] == 999)
        {
          self.game.EditObj.SetViewMode2 = 0;
          windowReturnClass.AddCommand(1, 9);
          windowReturnClass.AddCommand(7, 12);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index] && self.SubPartID[index] == self.Info1Id)
        {
          self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
          self.SubPartFlag[index] = true;
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      if (x > 8 & x < self.w - 8 && y < self.h - 24)
        windowReturnClass.NoMouseClickBelow = true;
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
