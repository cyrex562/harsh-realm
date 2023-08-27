// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.WelcomeWindowClass
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
  pub class WelcomeWindowClass : WindowClass
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

    pub fn PopUpRefresh() => self.DoRefresh();

    pub WelcomeWindowClass(
       tGame: GameClass,
      bool tmenudirect,
      ScreenClass tscreen,
      bool MarcStyle)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND4MARC)
    {
      self.opti = new int[2];
      self.txt = new int[2];
      self.tempBlink = 0.0f;
      self.detailnr = -1;
      self.game.EditObj.TutMode = false;
      self.game.EditObj.TutStep = 0;
      self.game.EditObj.TutOrder = -1;
      self.game.AIRunning = false;
      self.game.EditObj.AIMoving = false;
      self.opti = new int[self.game.ModCounter + 1];
      self.txt = new int[self.game.ModCounter + 1];
      self.subphase = 0;
      self.phase = 0;
      if (tmenudirect)
        self.menudirect = true;
      if (self.game.EditObj.IntroSoundOn)
        SoundMod.PlayEventBackground(self.game.AppPath + "sound/" + self.game.ModOpeningSoundtrack,  self.game.EditObj);
      self.DoStuff2();
    }

    pub WelcomeWindowClass( tGame: GameClass, bool tmenudirect, ScreenClass tscreen)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND4MARC)
    {
      self.opti = new int[2];
      self.txt = new int[2];
      self.tempBlink = 0.0f;
      self.detailnr = -1;
      self.game.EditObj.TutMode = false;
      self.game.EditObj.TutStep = 0;
      self.game.EditObj.TutOrder = -1;
      self.opti = new int[self.game.ModCounter + 1];
      self.txt = new int[self.game.ModCounter + 1];
      self.subphase = 0;
      self.phase = 0;
      if (tmenudirect)
        self.menudirect = true;
      if (self.game.EditObj.IntroSoundOn)
        SoundMod.PlayEventBackground(self.game.AppPath + "sound/" + self.game.ModOpeningSoundtrack,  self.game.EditObj);
      self.DoStuff();
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

    pub fn DoStuff()
    {
      SizeF sizeF = SizeF::new();
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      objgraphics.SmoothingMode = SmoothingMode.AntiAlias;
      objgraphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      objgraphics.TextContrast = 1;
      tstring: String = "v " + Conversion.Str( 110) + Strings.Trim(".04b");
      DrawMod.DrawTextColouredMarc( objgraphics, tstring, self.game.GameFont1, 5, 5, Color.White);
      self.dobuttons(0);
    }

    pub fn DoStuff2()
    {
      SizeF sizeF1 = SizeF::new();
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
       let mut local1: &Graphics = &graphics;
      bitmap: Bitmap = BitmapStore.GetBitmap(self.game.MARCINTRO2);
       let mut local2: &Bitmap = &bitmap;
      let mut x: i32 =  Math.Round( (1024 - BitmapStore.GetWidth(self.game.MARCINTRO2)) / 2.0);
      DrawMod.DrawSimple( local1,  local2, x, 50);
      str1: String = "v " + Strings.Trim(Conversion.Str( 110)) + Strings.Trim(".04b");
      DrawMod.DrawTextColouredMarc( graphics, str1, self.game.GameFont1, 5, 5, Color.White);
      SizeF sizeF2 = graphics.MeasureString(str1, self.game.GameFont1);
      DrawMod.DrawBlock( graphics, 5, 25,  Math.Round( sizeF2.Width),  Math.Round( sizeF2.Height),  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      DrawMod.DrawTextColoured( graphics, str1, self.game.GameFont1, 5, 25, Color.Black);
      str2: String = "Please remember: Scenarios saved with this editor can only be loaded in this editor! Distribution has to go through Vic! :)";
      SizeF sizeF3 = graphics.MeasureString(str2, self.game.GameFont1);
      DrawMod.DrawBlock( graphics, 5, 50,  Math.Round( sizeF3.Width),  Math.Round( sizeF3.Height), 0, 0, 200,  byte.MaxValue);
      DrawMod.DrawTextColoured( graphics, str2, self.game.GameFont1, 5, 50, Color.White);
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
          bool flag1 = true;
          if (self.game.ModButActive[index1] == 0)
            flag1 = false;
          bool flag2 = DrawMod.TGame.ModIntroType != 0;
          if (self.game.ModButSize[index1] == 18)
          {
            tfont: Font = self.game.MarcFont4;
            if (!flag2)
              tfont = Font::new("Arial", 14f, FontStyle.Regular, GraphicsUnit.Pixel);
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
          else if (self.game.ModButSize[index1] == 8)
          {
            usefont: Font = self.game.MarcFont4;
            if (!flag2)
              usefont =  null;
            int[] opti = self.opti;
            let mut index2: i32 = index1;
            let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 200, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 25, usefont: usefont, useshadow: flag2, tMarcStyle: flag2);
            let mut num: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 200, 25, 1);
            opti[index2] = num;
          }
          else if (self.game.ModButSize[index1] == 7)
          {
            usefont: Font = self.game.MarcFont3;
            if (!flag2)
              usefont =  null;
            int[] opti = self.opti;
            let mut index3: i32 = index1;
            let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 200, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 30, usefont: usefont, useshadow: flag2, tMarcStyle: flag2);
            let mut num: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 200, 30, 1);
            opti[index3] = num;
          }
          else if (self.game.ModButSize[index1] == 6)
          {
            usefont: Font = self.game.MarcFont2;
            if (!flag2)
              usefont =  null;
            int[] opti = self.opti;
            let mut index4: i32 = index1;
            let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 250, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 50, usefont: usefont, useshadow: flag2, tMarcStyle: flag2);
            let mut num: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 250, 50, 1);
            opti[index4] = num;
          }
          else if (self.game.ModButSize[index1] == 5)
          {
            usefont: Font = self.game.MarcFont1;
            if (!flag2)
              usefont =  null;
            int[] opti = self.opti;
            let mut index5: i32 = index1;
            let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 250, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 50, usefont: usefont, useshadow: flag2, tMarcStyle: flag2);
            let mut num: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 250, 50, 1);
            opti[index5] = num;
          }
          else if (self.game.ModButSize[index1] == 4)
          {
            int[] opti = self.opti;
            let mut index6: i32 = index1;
            let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 400, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 50, tfontsize: 16, tMarcStyle: flag2);
            let mut num: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 400, 50, 1);
            opti[index6] = num;
          }
          else if (self.game.ModButSize[index1] == 3)
          {
            int[] opti = self.opti;
            let mut index7: i32 = index1;
            let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 300, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), tfontsize: 14, tMarcStyle: flag2);
            let mut num: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 300, 35, 1);
            opti[index7] = num;
          }
          else if (self.game.ModButSize[index1] == 2)
          {
            int[] opti = self.opti;
            let mut index8: i32 = index1;
            let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 200, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 25, tfontsize: 12, tMarcStyle: flag2);
            let mut num: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 200, 25, 1);
            opti[index8] = num;
          }
          else if (self.game.ModButSize[index1] == 1)
          {
            int[] opti = self.opti;
            let mut index9: i32 = index1;
            let mut tsubpart: SubPartClass =  new TextButtonPartClass(self.game.ModButText[index1], 100, tBackbitmap: ( self.BackBitmap), bbx: ( Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1])), bby: (top + self.game.ModButY[index1]), tinactive: (!flag1), theight: 15, tfontsize: 10, tMarcStyle: flag2);
            let mut num: i32 = self.AddSubPart( tsubpart,  Math.Round( self.OwnBitmap.Width / 2.0 +  self.game.ModButX[index1]), top + self.game.ModButY[index1], 100, 15, 1);
            opti[index9] = num;
          }
        }
      }
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.game.ModIntroType == 1)
        return windowReturnClass;
      if (self.subphase < 9999)
      {
        self += 1.subphase;
        self.DoStuff();
      }
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
              if (self.game.ModButType[index2] == 5)
              {
                SoundMod.StopWave();
                SoundMod.EndSound();
                self.game = (GameClass) null;
                ProjectData.EndApp();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.ModButType[index2] == 14)
              {
                DrawMod.TGame.EditObj.PopupValue = 9;
                windowReturnClass1.AddCommand(5, 10);
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
                  let mut num: i32 =  Interaction.MsgBox( "Your system does not allow Shadow Empire : Planetary Conquest to open a browser.", Title: ( "Shadow Empire : Planetary Conquest"));
                  ProjectData.ClearProjectError();
                }
                return windowReturnClass1;
              }
              if (self.game.ModButType[index2] == 11)
              {
                self.game.EditObj.RandomUseMaster = self.game.ModButDatastring[index2];
                self.game.EditObj.RandomSettingsFromMod = self.game.ModButDatastring2[index2];
                self.game.EditObj.ShortRandomScreen = false;
                windowReturnClass1.AddCommand(1, 49);
                windowReturnClass1.AddCommand(2, 50);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.ModButType[index2] == 13)
              {
                self.game.EditObj.RandomUseMaster = self.game.ModButDatastring[index2];
                self.game.EditObj.RandomSettingsFromMod = self.game.ModButDatastring2[index2];
                self.game.EditObj.ShortRandomScreen = true;
                windowReturnClass1.AddCommand(1, 49);
                windowReturnClass1.AddCommand(2, 50);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.ModButType[index2] == 15)
              {
                self.Import();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (self.game.ModButType[index2] == 12)
              {
                str1: String = Conversions.ToString(Conversion.Val(Interaction.InputBox("Give width of map in hexes (10-200)", "Shadow Empire : Planetary Conquest", "20")));
                let mut twidth: i32 = Operators.CompareString(Strings.Trim(str1), "", false) == 0 ? 0 : Conversions.ToInteger(str1);
                str2: String = Conversions.ToString(Conversion.Val(Interaction.InputBox("Give height of map in hexes (10-200)", "Shadow Empire : Planetary Conquest", "20")));
                let mut theight: i32 = Operators.CompareString(Strings.Trim(str2), "", false) == 0 ? 0 : Conversions.ToInteger(str2);
                if (twidth < 10 | theight < 10 | twidth > 200 | theight > 200)
                {
                  let mut num: i32 =  Interaction.MsgBox( "Cannot comply. Width and Height must be between 10 and 200", Title: ( "Shadow Empire : Planetary Conquest"));
                  return windowReturnClass1;
                }
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
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
              if (self.game.ModButType[index2] <= 4)
              {
                str: String = self.game.ModButType[index2] > 2 ? (self.game.ModButType[index2] != 3 ? self.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a scenario to load...", self.game.AppPath_SAVEGAMES, false) : self.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a scenario to load...", self.game.AppPath + self.game.ModButDatastring[index2], false)) : self.game.AppPath + self.game.ModButDatastring[index2];
                if (File.Exists(str))
                {
                  self.game.FormRef.Cursor = Cursors.WaitCursor;
                  self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
                  if (self.game.ModButType[index2] == 2)
                    self.game.EditObj.TutMode = true;
                  else
                    self.game.EditObj.TutMode = false;
                  if (self.game.Data.UseAI == 1)
                    self.game.NewAIObj.LastRegime = -1;
                  self.game.SelectX = -1;
                  self.game.SelectY = -1;
                  self.game.Data = DataClass::new();
                  GC.Collect();
                  Application.DoEvents();
                  self.game.HandyFunctionsObj.Unzip(str);
                  self.game.Data = DataClass.deserialize(str);
                  self.game.HandyFunctionsObj.ZipFile(str);
                  if (Strings.Len(self.game.Data.MasterFile) > 0 & self.game.Data.Round == 0)
                  {
                    masterFile: String = self.game.Data.MasterFile;
                    self.game.HandyFunctionsObj.LoadMasterFile(self.game.HandyFunctionsObj.ReturnLongMaster(str, masterFile));
                  }
                  if ( self.game.Data.RuleVar[344] == 1.0 & self.game.EditObj.HideUnit == 0)
                    self.game.EditObj.HideUnit = 2;
                  self.game.EditObj.TempValue = new MapMatrix2[self.game.Data.MapCounter + 1];
                  self.game.EditObj.TempValue2 = new MapMatrix2[self.game.Data.MapCounter + 1];
                  let mut mapCounter: i32 = self.game.Data.MapCounter;
                  for (let mut index3: i32 = 0; index3 <= mapCounter; index3 += 1)
                  {
                    self.game.EditObj.TempValue[index3] = new MapMatrix2(self.game.Data.MapObj[index3].MapWidth, self.game.Data.MapObj[index3].MapHeight);
                    self.game.EditObj.TempValue2[index3] = new MapMatrix2(self.game.Data.MapObj[index3].MapWidth, self.game.Data.MapObj[index3].MapHeight);
                  }
                  if (self.game.Data.Round > 0)
                  {
                    BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                    self.game.Data.LoadGraphics((Form1) null);
                    self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                    self.game.EditObj.StratMap = new Bitmap(self.game.ScreenWidth, self.game.ScreenHeight - 305);
                    self.game.EditObj.StratMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
                    self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.StratMap, self.game.ScreenWidth, self.game.ScreenHeight - 305, false, true, false);
                    SoundMod.StopWave();
                    self.game.FormRef.Cursor = Cursors.Default;
                    if (!self.game.Data.InTurn)
                    {
                      windowReturnClass1.AddCommand(3, 4);
                      self.game.EditObj.Phase = -1;
                    }
                    else
                    {
                      self.game.HandyFunctionsObj.SetInitialXY(self.game.Data.Turn);
                      self.game.FormRef.Cursor = Cursors.Default;
                      self.game.EventRelatedObj.DoCheckEvents(4);
                      self.game.ProcessingObj.LocationProductionPrognosis();
                      if ( self.game.Data.RuleVar[839] == 0.0)
                        windowReturnClass1.AddCommand(3, 3);
                      else
                        windowReturnClass1.AddCommand(3, 11);
                    }
                    return windowReturnClass1;
                  }
                  if (Strings.Len(self.game.Data.LoadPass) > 0)
                  {
                    self.game.FormRef.Cursor = Cursors.Default;
                    if (Operators.CompareString(Strings.LCase(Interaction.InputBox("This File is protected by a load password. Please give it in order to load it.", "Shadow Empire : Planetary Conquest")), Strings.LCase(self.game.Data.LoadPass), false) == 0)
                    {
                      let mut num1: i32 =  Interaction.MsgBox( "You are cleared.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    else
                    {
                      let mut num2: i32 =  Interaction.MsgBox( "Wrong Password. You cannot Load this file", Title: ( "Shadow Empire : Planetary Conquest"));
                      self.game.Data = DataClass::new();
                      self.RemoveSubPart(self.TempText);
                      let mut tsubpart: SubPartClass =  TextPartClass::new(self.game.Data.Name + " is loaded instead.", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, true);
                      self.TempText = self.AddSubPart( tsubpart, 0, 41, 400, 19, 0);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                  }
                  BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                  self.game.Data.LoadGraphics((Form1) null);
                  DrawMod.TGame = self.game;
                  self.game.FormRef.Cursor = Cursors.Default;
                  self.RemoveSubPart(self.TempText);
                  windowReturnClass1.AddCommand(3, 1);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (Strings.Len(str) > 1)
                {
                  let mut num: i32 =  Interaction.MsgBox( "File could not be found or op. is cancelled.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                return windowReturnClass1;
              }
            }
          }
          return windowReturnClass1;
        }
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    pub fn Import()
    {
      str: String = self.game.HandyFunctionsObj.LoadSomething("AT Zip Package File (*.atzip)|*.atzip|AT2 file(*.at2)|*.at2|Regular Zip(*.zip)|*.zip", "Pick a zip archive or scenario file to install.", self.game.AppPath, false);
      if (Information.IsNothing( str))
        return;
      if (Strings.InStr(str, ".zip") > 0 | Strings.InStr(str, ".atzip") > 0)
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
          let mut num2: i32 =  Interaction.MsgBox( "Error in unpacking. ", Title: ( "Shadow Empire : Planetary Conquest"));
          ProjectData.ClearProjectError();
          return;
        }
        self.game.FormRef.Cursor = Cursors.Default;
        let mut num3: i32 =  Interaction.MsgBox( "Succesfully unpacked the .zip archive.", Title: ( "Shadow Empire : Planetary Conquest"));
      }
      else
      {
        if (Information.IsNothing( str))
          return;
        path: String = self.game.AppPath + self.game.ModScenarioDir + "/DOWNLOADED_SCENARIOS/";
        if (!Directory.Exists(path))
          Directory.CreateDirectory(path);
        destFileName: String = path + Path.GetFileName(str);
        File.Copy(str, destFileName, true);
        let mut num: i32 =  Interaction.MsgBox( "Succesfully placed the file in the downloaded scenarios directory.", Title: ( "Shadow Empire : Planetary Conquest"));
      }
    }
  }
}
