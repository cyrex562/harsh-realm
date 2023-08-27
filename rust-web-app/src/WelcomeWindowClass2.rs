// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.WelcomeWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Diagnostics;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;
// usingSystem.IO;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class WelcomeWindowClass2 : WindowClass
  {
     BStartGameID: i32;
     BLoadGameID: i32;
     BSaveGameID: i32;
     BRandomID: i32;
     BEditorID: i32;
     TempText: i32;
     TempText2: i32;
     txt1: i32;
     txt2: i32;
     txt3: i32;
     opt1: i32;
     opt2: i32;
     opt3: i32;
     opt4: i32;
     opt5: i32;
     opt6: i32;
     opt7: i32;
     opt8: i32;
     opt9: i32;
     opt10: i32;
     opt11: i32;
     opt12: i32;
     opt13: i32;
     opt14: i32;
     opt15: i32;
     opt16: i32;
     opt17: i32;
     opt18: i32;
     opt19: i32;
     opt20: i32;
     txt4: i32;
     cancelID: i32;
     ListClass RegimeListObj;
     RegimeListId: i32;
     float tempBlink;
     detailnr: i32;
     phase: i32;
     subphase: i32;
     bool menudirect;
     int[] opti;
     int[] txt;
     bool flagCallToSSeditor;

    pub fn PopUpRefresh() => self.DoRefresh();

    pub WelcomeWindowClass2(
       tGame: GameClass,
      bool tmenudirect,
      ScreenClass tscreen,
      bool MarcStyle)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND1MARC2)
    {
      self.opti = new int[2];
      self.txt = new int[2];
      self.flagCallToSSeditor = false;
      self.tempBlink = 0.0f;
      self.detailnr = -1;
      self.game.EditObj.TutStep = 0;
      self.game.EditObj.TutOrder = -1;
      self.game.EditObj.LoadingResult = LoadType.None;
      self.game.AIRunning = false;
      self.game.EditObj.AIMoving = false;
      self.opti = new int[self.game.ModCounter + 1];
      self.txt = new int[self.game.ModCounter + 1];
      self.subphase = 0;
      self.phase = 0;
      if (tmenudirect)
        self.menudirect = true;
      self.game.Data.RuleVar[446] = 0.0f;
      if (self.game.EditObj.IntroSoundOn)
        SoundMod.PlayEventBackground(self.game.AppPath + "sound/" + self.game.ModOpeningSoundtrack,  self.game.EditObj);
      self.DoStuff2();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
      {
        if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
        {
          let mut modCounter: i32 = self.game.ModCounter;
          for (let mut index2: i32 = 0; index2 <= modCounter; index2 += 1)
          {
            if (self.opti[index2] == self.SubPartID[index1])
            {
              if (self.game.ModButType[index2] == 5)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "QUIT";
                self.game.EditObj.TipText = "Close the game and return to desktop.";
              }
              if (self.game.ModButType[index2] == 16)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "PBEM++";
                self.game.EditObj.TipText = "Got the PBEM++ screen to start or continue a PBEM++ game.";
              }
              if (self.game.ModButType[index2] == 6)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "OPEN WEBSITE";
                self.game.EditObj.TipText = "Will attempt to open:\r\n" + self.game.ModButDatastring[index2];
              }
              if (self.game.ModButType[index2] == 14)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "SEE CREDITS";
                self.game.EditObj.TipText = "Will show you a list of VR Designs and Matrix Games credits";
              }
              if (self.game.ModButType[index2] == 2)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "LOAD SPECIFIC TUTORIAL";
                self.game.EditObj.TipText = "Will attempt to load tutorial:\r\n" + self.game.ModButDatastring[index2];
              }
              if (self.game.ModButType[index2] == 1)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "LOAD SPECIFIC SCENARIO";
                self.game.EditObj.TipText = "Will attempt to load scenario:\r\n" + self.game.ModButDatastring[index2];
              }
              if (self.game.ModButType[index2] == 3)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "LOAD SCENARIO";
                self.game.EditObj.TipText = "Will allow you to select a scenario to load.";
              }
              if (self.game.ModButType[index2] == 12)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "ADVANCED EDITOR";
                self.game.EditObj.TipText = "Will open the advanced editor.";
              }
              if (self.game.ModButType[index2] == 19)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "SIMPLE EDITOR";
                self.game.EditObj.TipText = "Will open the simple editor.";
              }
              if (self.game.ModButType[index2] == 20)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "TROOPTYPE EDITOR";
                self.game.EditObj.TipText = "Will open the trooptype editor.";
              }
              if (self.game.ModButType[index2] == 21)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "HISTORICAL EDITOR";
                self.game.EditObj.TipText = "Will open the historical units and models editor.";
              }
              if (self.game.ModButType[index2] == 22)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "OFFICER EDITOR";
                self.game.EditObj.TipText = "Will open the officer editor.";
              }
              if (self.game.ModButType[index2] == 23)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "MAP EDITOR";
                self.game.EditObj.TipText = "Will open the map editor.";
              }
              if (self.game.ModButType[index2] == 4)
              {
                self.game.EditObj.TipButton = true;
                self.game.EditObj.TipTitle = "LOAD SAVED GAME";
                self.game.EditObj.TipText = "Will allow you to select a saved game to load and continue.";
              }
            }
          }
        }
      }
    }

    pub fn DoStuff2()
    {
      SizeF sizeF = SizeF::new();
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      objgraphics.SmoothingMode = SmoothingMode.AntiAlias;
      objgraphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      objgraphics.TextContrast = 1;
      if (self.game.Data.Product == 6)
      {
         let mut local1: &Graphics = &objgraphics;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCINTRO2);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 =  Math.Round( (1024 - BitmapStore.GetWidth(self.game.MARCINTRO2)) / 2.0);
        DrawMod.DrawSimple( local1,  local2, x, 50);
      }
      else
      {
         let mut local3: &Graphics = &objgraphics;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCINTRO2);
         let mut local4: &Bitmap = &bitmap;
        let mut x: i32 =  Math.Round( (1024 - BitmapStore.GetWidth(self.game.MARCINTRO2)) / 2.0);
        DrawMod.DrawSimple( local3,  local4, x, 50);
      }
      tstring: String = "v " + Strings.Trim(Conversion.Str( Math.Floor(1.1))) + "." + Strings.Trim(Conversion.Str( 10)) + " " + Strings.Trim(".04b") + " Shadow Empire : Planetary Conquest. ";
      if (self.game.SuperAdminRights)
        tstring += " + Super Admin Rights";
      DrawMod.DrawTextColouredMarc( objgraphics, tstring, self.game.MarcFont4, 25, 732, Color.White);
      self.dobuttons(0);
    }

    pub fn dobuttons(top: i32)
    {
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      let mut modCounter: i32 = self.game.ModCounter;
      for (let mut index1: i32 = 1; index1 <= modCounter; index1 += 1)
      {
        if (self.opti[index1] == 0)
        {
          let mut num1: i32 = 1;
          if (self.game.ModButType[index1] == 11)
            num1 = 0;
          if (self.game.ModButType[index1] == 13)
            num1 = 0;
          if (self.game.EditorBlock & self.game.ModButType[index1] == 12)
            num1 = 0;
          if (self.game.EditorBlockSimple & self.game.ModButType[index1] == 19)
            num1 = 0;
          if (self.game.EditorBlockSimple & self.game.ModButType[index1] == 20)
            num1 = 0;
          if (self.game.EditorBlockSimple & self.game.ModButType[index1] == 21)
            num1 = 0;
          if (self.game.EditorBlockSimple & self.game.ModButType[index1] == 22)
            num1 = 0;
          if (self.game.EditorBlockSimple & self.game.ModButType[index1] == 23)
            num1 = 0;
          if (num1 == 1)
          {
            bool flag1 = true;
            if (self.game.ModButActive[index1] == 0)
              flag1 = false;
            bool flag2 = DrawMod.TGame.ModIntroType != 0;
            if (self.game.ModButSize[index1] == 18)
            {
              tfont: Font = self.game.MarcFont4;
              if (!flag2)
                tfont = Font::new(self.game.FontCol.Families[1], 14f, FontStyle.Regular, GraphicsUnit.Pixel);
              if (flag2)
              {
                DrawMod.DrawTextColouredMarc( graphics, self.game.ModButText[index1], tfont,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], Color.White);
                DrawMod.DrawBlock( graphics,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1] + 20, 200, 2,  byte.MaxValue, 0, 0, 200);
              }
              else
              {
                DrawMod.DrawTextColouredOutline( graphics, self.game.ModButText[index1], tfont,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], Color.White);
                DrawMod.DrawBlock( graphics,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]) + 1, top + self.game.ModButY[index1] + 21, 200, 2,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 200);
                DrawMod.DrawBlock( graphics,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1] + 20, 200, 2,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 200);
              }
            }
            else if (self.game.ModButSize[index1] == 9)
            {
              usefont: Font = self.game.MarcFont2;
              if (!flag2)
                usefont =  null;
              int[] opti = self.opti;
              let mut index2: i32 = index1;
              let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 200, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 50, usefont: usefont, useshadow: flag2, tMarcStyle: flag2);
              let mut num2: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 200, 50, 1);
              opti[index2] = num2;
            }
            else if (self.game.ModButSize[index1] == 8)
            {
              usefont: Font = self.game.MarcFont4;
              if (!flag2)
                usefont =  null;
              int[] opti = self.opti;
              let mut index3: i32 = index1;
              let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 200, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 25, usefont: usefont, useshadow: flag2, tMarcStyle: flag2);
              let mut num3: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 200, 25, 1);
              opti[index3] = num3;
            }
            else if (self.game.ModButSize[index1] == 7)
            {
              usefont: Font = self.game.MarcFont3;
              if (!flag2)
                usefont =  null;
              int[] opti = self.opti;
              let mut index4: i32 = index1;
              let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 200, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 30, usefont: usefont, useshadow: flag2, tMarcStyle: flag2);
              let mut num4: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 200, 30, 1);
              opti[index4] = num4;
            }
            else if (self.game.ModButSize[index1] == 6)
            {
              usefont: Font = self.game.MarcFont2;
              if (!flag2)
                usefont =  null;
              int[] opti = self.opti;
              let mut index5: i32 = index1;
              let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 250, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 50, usefont: usefont, useshadow: flag2, tMarcStyle: flag2);
              let mut num5: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 250, 50, 1);
              opti[index5] = num5;
            }
            else if (self.game.ModButSize[index1] == 5)
            {
              usefont: Font = self.game.MarcFont1;
              if (!flag2)
                usefont =  null;
              int[] opti = self.opti;
              let mut index6: i32 = index1;
              let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 250, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 50, usefont: usefont, useshadow: flag2, tMarcStyle: flag2);
              let mut num6: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 250, 50, 1);
              opti[index6] = num6;
            }
            else if (self.game.ModButSize[index1] == 4)
            {
              int[] opti = self.opti;
              let mut index7: i32 = index1;
              let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 400, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 50, tfontsize: 16, tMarcStyle: flag2);
              let mut num7: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 400, 50, 1);
              opti[index7] = num7;
            }
            else if (self.game.ModButSize[index1] == 3)
            {
              int[] opti = self.opti;
              let mut index8: i32 = index1;
              let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 300, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), tfontsize: 14, tMarcStyle: flag2);
              let mut num8: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 300, 35, 1);
              opti[index8] = num8;
            }
            else if (self.game.ModButSize[index1] == 2)
            {
              int[] opti = self.opti;
              let mut index9: i32 = index1;
              let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 200, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 25, tfontsize: 12, tMarcStyle: flag2);
              let mut num9: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 200, 25, 1);
              opti[index9] = num9;
            }
            else if (self.game.ModButSize[index1] == 1)
            {
              int[] opti = self.opti;
              let mut index10: i32 = index1;
              let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 100, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 15, tfontsize: 10, tMarcStyle: flag2);
              let mut num10: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 100, 15, 1);
              opti[index10] = num10;
            }
          }
        }
      }
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.game.EditObj.LoadingResult == LoadType.RandomScreen)
      {
        windowReturnClass.AddCommand(3, 23);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (self.flagCallToSSeditor)
      {
        self.flagCallToSSeditor = false;
        windowReturnClass.AddCommand(3, 25);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (self.game.EditObj.LoadingResult == LoadType.FirstScreen)
      {
        self.game.EditObj.LoadingResult = LoadType.None;
        if (".se1map".Length > 0 & Strings.InStr(self.game.EditObj.LoadFileName, ".se1map") > 0)
          windowReturnClass.AddCommand(3, 21);
        else if (".se1evlib".Length > 0 & Strings.InStr(self.game.EditObj.LoadFileName, ".se1evlib") > 0)
        {
          windowReturnClass.AddCommand(3, 12);
          self.game.Data.SimpleEditor = false;
        }
        else if (".se1his".Length > 0 & Strings.InStr(self.game.EditObj.LoadFileName, ".se1his") > 0)
          windowReturnClass.AddCommand(3, 19);
        else if (".se1offcard".Length > 0 & Strings.InStr(self.game.EditObj.LoadFileName, ".se1offcard") > 0)
        {
          windowReturnClass.AddCommand(3, 12);
          self.game.Data.SimpleEditor = false;
        }
        else if (".se1off".Length > 0 & Strings.InStr(self.game.EditObj.LoadFileName, ".se1off") > 0)
          windowReturnClass.AddCommand(3, 20);
        else if (".se1troops".Length > 0 & Strings.InStr(self.game.EditObj.LoadFileName, ".se1troops") > 0)
          windowReturnClass.AddCommand(3, 18);
        else if (".se1master".Length > 0 & Strings.InStr(self.game.EditObj.LoadFileName, ".se1master") > 0)
        {
          windowReturnClass.AddCommand(3, 12);
          self.game.Data.SimpleEditor = false;
        }
        else
          windowReturnClass.AddCommand(3, 12);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (self.game.EditObj.LoadingResult == LoadType.PlayScreen)
      {
        self.game.EditObj.LoadingResult = LoadType.None;
        windowReturnClass.AddCommand(3, 11);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (self.game.EditObj.LoadingResult != LoadType.GameLoop)
        return windowReturnClass;
      self.game.EditObj.LoadingResult = LoadType.None;
      windowReturnClass.AddCommand(3, 13);
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false) => WindowReturnClass::new();

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.NoOptimization)]
    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter <= -1)
      {
        windowReturnClass2: WindowReturnClass;
        return windowReturnClass2;
      }
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
      {
        if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
        {
          let mut modCounter: i32 = self.game.ModCounter;
          for (let mut index2: i32 = 1; index2 <= modCounter; index2 += 1)
          {
            if (self.opti[index2] == self.SubPartID[index1])
            {
              self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              if (self.game.ModButType[index2] == 5)
              {
                SoundMod.StopWave();
                SoundMod.EndSound();
                self.game = (GameClass) null;
                ProjectData.EndApp();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.ModButType[index2] == 16)
              {
                let mut num1: i32 =  Interaction.MsgBox( "PBEM++ not supported for this game", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                if (self.game.ModButType[index2] == 15)
                {
                  self.ImportSe1Zip();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.game.ModButType[index2] == 14)
                {
                  DrawMod.TGame.EditObj.PopupValue = 9;
                  windowReturnClass1.AddCommand(5, 14);
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.game.ModButType[index2] == 24)
                {
                  DrawMod.TGame.EditObj.PopupValue = 20;
                  windowReturnClass1.AddCommand(5, 14);
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.game.ModButType[index2] == 34)
                {
                  DrawMod.TGame.EditObj.PopupValue = 34;
                  windowReturnClass1.AddCommand(5, 14);
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.game.ModButType[index2] == 6)
                {
                  try
                  {
                    Process.Start(self.game.ModButDatastring[index2]);
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    let mut num2: i32 =  Interaction.MsgBox( "Your system does not allow Shadow Empire : Planetary Conquest to open a browser.", Title: ( "Shadow Empire : Planetary Conquest"));
                    ProjectData.ClearProjectError();
                  }
                  return windowReturnClass1;
                }
                if (self.game.ModButType[index2] != 11 && self.game.ModButType[index2] != 13)
                {
                  if (self.game.ModButType[index2] == 12)
                  {
                    str1: String = Conversions.ToString(Conversion.Val(Interaction.InputBox("Give width of map in hexes (10-200)", "Shadow Empire : Planetary Conquest", "20")));
                    let mut twidth: i32 = Operators.CompareString(Strings.Trim(str1), "", false) == 0 ? 0 : Conversions.ToInteger(str1);
                    str2: String = Conversions.ToString(Conversion.Val(Interaction.InputBox("Give height of map in hexes (10-200)", "Shadow Empire : Planetary Conquest", "20")));
                    let mut theight: i32 = Operators.CompareString(Strings.Trim(str2), "", false) == 0 ? 0 : Conversions.ToInteger(str2);
                    if (twidth < 10 | theight < 10 | twidth > 200 | theight > 200)
                    {
                      let mut num3: i32 =  Interaction.MsgBox( "Cannot comply. Width and Height must be between 10 and 200", Title: ( "Shadow Empire : Planetary Conquest"));
                      return windowReturnClass1;
                    }
                    self.game.FormRef.Cursor = Cursors.WaitCursor;
                    self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
                    if (Information.IsNothing( self.game.NewAIObj))
                      self.game.NewAIObj = new NewAIClass(self.game);
                    if (self.game.Data.UseAI == 1)
                      self.game.NewAIObj.LastRegime = -1;
                    self.game.Data = new DataClass(twidth, theight);
                    self.game.Data.MasterFile = self.game.ModButDatastring[index2];
                    self.game.HandyFunctionsObj.LoadMasterFile(self.game.AppPath + self.game.ModScenarioDir + "/" + self.game.Data.MasterFile);
                    BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                    self.game.Data.LoadGraphics((Form1) null);
                    self.game.SelectX = 0;
                    self.game.SelectY = 0;
                    self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                    self.game.FormRef.Cursor = Cursors.Default;
                    SoundMod.StopWave();
                    windowReturnClass1.AddCommand(3, 2);
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  if (self.game.ModButType[index2] == 19)
                  {
                    self.game.FormRef.Cursor = Cursors.WaitCursor;
                    self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
                    if (self.game.Data.UseAI == 1)
                    {
                      if (Information.IsNothing( self.game.NewAIObj))
                        self.game.NewAIObj = new NewAIClass(self.game);
                      self.game.NewAIObj.LastRegime = -1;
                    }
                    self.game.Data = new DataClass(0, 0);
                    self.game.Data.MasterFile = self.game.ModButDatastring[index2];
                    self.game.HandyFunctionsObj.LoadMasterFile(self.game.AppPath + self.game.ModScenarioDir + "/" + self.game.Data.MasterFile);
                    BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                    self.game.Data.LoadGraphics((Form1) null);
                    self.game.SelectX = 0;
                    self.game.SelectY = 0;
                    self.game.Data.SimpleEditor = true;
                    self.game.EditObj.inSimpleMapEditor = false;
                    self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                    self.game.FormRef.Cursor = Cursors.Default;
                    SoundMod.StopWave();
                    windowReturnClass1.AddCommand(3, 17);
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  if (self.game.ModButType[index2] == 31)
                  {
                    str: String = self.game.AppPath + self.game.ModScenarioDir + "/" + self.game.ModButDatastring[index2];
                    self.game.FormRef.Cursor = Cursors.WaitCursor;
                    self.game.EditObj.systemPopup = false;
                    self.game.EditObj.TutMode = false;
                    if (File.Exists(str))
                    {
                      self.flagCallToSSeditor = true;
                      self.game.EditObj.LoadFileName = str;
                      self.game.EditObj.PopupValue = 17;
                      windowReturnClass1.AddCommand(5, 14);
                      self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (Strings.Len(str) > 1)
                    {
                      let mut num4: i32 =  Interaction.MsgBox( "File could not be found or op. is cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    return windowReturnClass1;
                  }
                  if (self.game.ModButType[index2] == 20)
                  {
                    self.game.FormRef.Cursor = Cursors.WaitCursor;
                    self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
                    if (self.game.Data.UseAI == 1)
                    {
                      if (Information.IsNothing( self.game.NewAIObj))
                        self.game.NewAIObj = new NewAIClass(self.game);
                      self.game.NewAIObj.LastRegime = -1;
                    }
                    self.game.Data = new DataClass(0, 0);
                    self.game.Data.MasterFile = self.game.ModButDatastring[index2];
                    self.game.HandyFunctionsObj.LoadMasterFile(self.game.AppPath + self.game.ModScenarioDir + "/" + self.game.Data.MasterFile);
                    BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                    self.game.Data.LoadGraphics((Form1) null);
                    self.game.SelectX = 0;
                    self.game.SelectY = 0;
                    self.game.Data.SimpleEditor = true;
                    self.game.EditObj.inSimpleMapEditor = false;
                    self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                    self.game.FormRef.Cursor = Cursors.Default;
                    SoundMod.StopWave();
                    windowReturnClass1.AddCommand(3, 18);
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  if (self.game.ModButType[index2] == 21)
                  {
                    self.game.FormRef.Cursor = Cursors.WaitCursor;
                    self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
                    if (self.game.Data.UseAI == 1)
                    {
                      if (Information.IsNothing( self.game.NewAIObj))
                        self.game.NewAIObj = new NewAIClass(self.game);
                      self.game.NewAIObj.LastRegime = -1;
                    }
                    self.game.Data = new DataClass(0, 0);
                    self.game.Data.MasterFile = self.game.ModButDatastring[index2];
                    self.game.HandyFunctionsObj.LoadMasterFile(self.game.AppPath + self.game.ModScenarioDir + "/" + self.game.Data.MasterFile);
                    BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                    self.game.Data.LoadGraphics((Form1) null);
                    self.game.SelectX = 0;
                    self.game.SelectY = 0;
                    self.game.Data.SimpleEditor = true;
                    self.game.EditObj.inSimpleMapEditor = false;
                    self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                    self.game.FormRef.Cursor = Cursors.Default;
                    SoundMod.StopWave();
                    windowReturnClass1.AddCommand(3, 19);
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  if (self.game.ModButType[index2] == 22)
                  {
                    self.game.FormRef.Cursor = Cursors.WaitCursor;
                    self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
                    if (self.game.Data.UseAI == 1)
                    {
                      if (Information.IsNothing( self.game.NewAIObj))
                        self.game.NewAIObj = new NewAIClass(self.game);
                      self.game.NewAIObj.LastRegime = -1;
                    }
                    self.game.Data = new DataClass(0, 0);
                    self.game.Data.MasterFile = self.game.ModButDatastring[index2];
                    self.game.HandyFunctionsObj.LoadMasterFile(self.game.AppPath + self.game.ModScenarioDir + "/" + self.game.Data.MasterFile);
                    BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                    self.game.Data.LoadGraphics((Form1) null);
                    self.game.SelectX = 0;
                    self.game.SelectY = 0;
                    self.game.Data.SimpleEditor = true;
                    self.game.EditObj.inSimpleMapEditor = false;
                    self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                    self.game.FormRef.Cursor = Cursors.Default;
                    SoundMod.StopWave();
                    windowReturnClass1.AddCommand(3, 20);
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  if (self.game.ModButType[index2] == 23)
                  {
                    self.game.FormRef.Cursor = Cursors.WaitCursor;
                    self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
                    if (self.game.Data.UseAI == 1)
                    {
                      if (Information.IsNothing( self.game.NewAIObj))
                        self.game.NewAIObj = new NewAIClass(self.game);
                      self.game.NewAIObj.LastRegime = -1;
                    }
                    self.game.Data = new DataClass(0, 0);
                    self.game.Data.MasterFile = self.game.ModButDatastring[index2];
                    self.game.HandyFunctionsObj.LoadMasterFile(self.game.AppPath + self.game.ModScenarioDir + "/" + self.game.Data.MasterFile);
                    BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                    self.game.Data.LoadGraphics((Form1) null);
                    self.game.SelectX = 0;
                    self.game.SelectY = 0;
                    self.game.Data.SimpleEditor = true;
                    self.game.EditObj.inSimpleMapEditor = true;
                    self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                    self.game.FormRef.Cursor = Cursors.Default;
                    SoundMod.StopWave();
                    windowReturnClass1.AddCommand(3, 21);
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  if (self.game.ModButType[index2] <= 4)
                  {
                    self.game.EditObj.systemPopup = true;
                    str: String;
                    if (self.game.ModButType[index2] <= 2)
                      str = self.game.AppPath + self.game.ModButDatastring[index2];
                    else if (self.game.ModButType[index2] == 3)
                    {
                      if (!self.game.EditorBlock & !self.game.EditorBlockSimple)
                      {
                        tfilter: String = "SE1 Scenario file (*.se1)|*.se1|SE1 Map file(*.se1map)|*.se1map|SE1 Master file(*.se1master)|*.se1master|SE1 Event library(*.se1evlib)|*.se1evlib|SE1 Troops&Equipment library(*.se1troops)|*.se1troops|SE1 Historical library(*.se1his)|*.se1his|SE1 Officer Card Library(*.se1offcard)|*.se1offcard|SE1 Officer library(*.se1off)|*.se1off";
                        if (self.game.SuperAdminRights)
                          tfilter += "|Any file(*.*)|*.*";
                        str = self.game.HandyFunctionsObj.LoadSomething(tfilter, "Pick a scenario to load...", self.game.AppPath + self.game.ModButDatastring[index2], false);
                      }
                      else
                        str = self.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a scenario to load...", self.game.AppPath + self.game.ModButDatastring[index2], false);
                    }
                    else
                      str = self.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a scenario to load...", self.game.AppPath_SAVEGAMES, false);
                    self.game.EditObj.systemPopup = false;
                    self.game.EditObj.TutMode = false;
                    if (File.Exists(str))
                    {
                      self.game.EditObj.LoadFileName = str;
                      if (self.game.ModButType[index2] == 2)
                        self.game.EditObj.TutMode = true;
                      if (self.game.ModButType[index2] == 1)
                        self.game.EditObj.ButtonLoadMode = true;
                      self.game.EditObj.PopupValue = 17;
                      windowReturnClass1.AddCommand(5, 14);
                      self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (Strings.Len(str) > 1)
                    {
                      let mut num5: i32 =  Interaction.MsgBox( "File could not be found or op. is cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    return windowReturnClass1;
                  }
                }
              }
            }
          }
          return windowReturnClass1;
        }
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.NoOptimization)]
    pub fn Import()
    {
      self.game.EditObj.FindModFile = false;
      self.game.EditObj.ZipFileText = Conversions.ToString(false);
      self.game.EditObj.systemPopup = true;
      str: String = self.game.HandyFunctionsObj.LoadSomething("All files|*.*", "Pick a zip archive or scenario file to install...", self.game.AppPath, false);
      self.game.EditObj.systemPopup = false;
      if (Information.IsNothing( str))
        return;
      if (Strings.InStr(str, ".zip") > 0 | Strings.InStr(str, ".dczip") > 0)
      {
        let mut num1: i32 =  Interaction.MsgBox( "Hold on... this can take some time", Title: ( "Shadow Empire : Planetary Conquest"));
        self.game.FormRef.Cursor = Cursors.WaitCursor;
        try
        {
          self.game.HandyFunctionsObj.UnzipImport(str);
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          let mut num2: i32 =  Interaction.MsgBox( ("Error in unpacking. " + ex.ToString()), Title: ( "Shadow Empire : Planetary Conquest"));
          ProjectData.ClearProjectError();
          return;
        }
        self.game.FormRef.Cursor = Cursors.Default;
        let mut num3: i32 =  Interaction.MsgBox( "Succesfully unpacked the .zip archive.", Title: ( "Shadow Empire : Planetary Conquest"));
        if (self.game.EditObj.ZipFileText.Length > 1)
        {
          let mut num4: i32 =  Interaction.MsgBox( ("README INCLUDED IN ZIPFILE:\r\n\r\n" + self.game.EditObj.ZipFileText), Title: ( "Shadow Empire : Planetary Conquest"));
        }
        if (self.game.EditObj.FindModFile && Interaction.MsgBox( "You are advised to quit now and restart the game. Do you want to do so?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
        {
          SoundMod.StopWave();
          SoundMod.EndSound();
          self.game = (GameClass) null;
          ProjectData.EndApp();
        }
      }
      else if (!Information.IsNothing( str))
      {
        path: String = self.game.AppPath + self.game.ModScenarioDir + "/DOWNLOADED_SCENARIOS/";
        if (!Directory.Exists(path))
          Directory.CreateDirectory(path);
        destFileName: String = path + Path.GetFileName(str);
        File.Copy(str, destFileName);
        let mut num: i32 =  Interaction.MsgBox( "Succesfully placed the file in the downloaded scenarios directory.", Title: ( "Shadow Empire : Planetary Conquest"));
      }
      BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx, true);
    }

    pub fn ImportSe1Zip()
    {
      self.game.EditObj.FindModFile = false;
      self.game.EditObj.ZipFileText = Conversions.ToString(false);
      self.game.EditObj.systemPopup = true;
      str: String = self.game.HandyFunctionsObj.LoadSomething("Se1Zip Files|*.se1zip|Normal Zip File|*.zip", "Pick a zip archive or scenario file to install...", self.game.AppPath, false);
      self.game.EditObj.systemPopup = false;
      if (Information.IsNothing( str) || !(Strings.InStr(str, ".zip") > 0 | Strings.InStr(str, ".se1zip") > 0))
        return;
      let mut num1: i32 =  Interaction.MsgBox( "Hold on... this can take some time", Title: ( "Shadow Empire : Planetary Conquest"));
      self.game.FormRef.Cursor = Cursors.WaitCursor;
      try
      {
        self.game.HandyFunctionsObj.UnzipImport(str);
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        let mut num2: i32 =  Interaction.MsgBox( ("Error in unpacking. " + ex.ToString()), Title: ( "Shadow Empire : Planetary Conquest"));
        ProjectData.ClearProjectError();
        return;
      }
      self.game.modlib_Initialize();
      self.game.modlib_loadPrefs();
      self.game.FormRef.Cursor = Cursors.Default;
      let mut num3: i32 =  Interaction.MsgBox( "Succesfully unpacked the archive.", Title: ( "Shadow Empire : Planetary Conquest"));
    }
  }
}
