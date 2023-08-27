// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TabHelpWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Diagnostics;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;

namespace WindowsApplication1
{
  pub class TabHelpWindowClass2 : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     ShowString: String;
     DateTime ShowTime;
     w: i32;
     h: i32;
     CurrentView: i32;
     currentHelp: i32;

    pub TabHelpWindowClass2(
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
      self.currentHelp = 0;
      self.dostuff();
    }

    pub fn DoRefresh() => self.dostuff();

    pub fn dostuff()
    {
      if (self.Info1Id > 0)
        self.RemoveSubPart(self.Info1Id);
      self.ClearMouse();
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      Rectangle trect1 = DrawMod.DrawBackTab(g, self.w, self.h, "HELP", 1);
      self.AddMouse( trect1, "CLOSE TAB", "Click here to close this tab. [ESC/F2]", 999);
      let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[988]));
      if (self.game.Data.StringListObj[stringListById].Length > -1)
      {
        g.SmoothingMode = SmoothingMode.None;
        DrawMod.drawLineDot( g, 135, 0, 135, self.h - 40, Color.White);
        DrawMod.drawLineDot( g, 15, 8, 135, 8, Color.White);
        let mut length: i32 = self.game.Data.StringListObj[stringListById].Length;
        for (let mut index: i32 = 0; index <= length; index += 1)
        {
          g.SmoothingMode = SmoothingMode.AntiAlias;
          if (self.currentHelp == -1)
            self.currentHelp = index;
          if (index == self.currentHelp)
            DrawMod.DrawBlockGradient( g, 35, 8 + 24 * index, 100, 24, Color.FromArgb(0, 0, 0, 0), Color.FromArgb(96,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
          Rectangle trect2 = Rectangle::new(15, 8 + 24 * index, 120, 24);
          self.AddMouse( trect2, "", "Click to get more info", 1000 + index);
          let mut y: i32 = 13 + 24 * index + 1;
          DrawMod.DrawTextColouredMarcCenter( g, self.game.Data.StringListObj[stringListById].Data[index, 0], self.game.MarcFont5, 75, y, Color.White);
          g.SmoothingMode = SmoothingMode.None;
          DrawMod.drawLineDot( g, 15, 8 + index * 24, 135, 8 + index * 24, Color.White);
        }
      }
      g.SmoothingMode = SmoothingMode.AntiAlias;
      tstring: String = self.game.Data.StringListObj[stringListById].Data[self.currentHelp, 1];
      str1: String = self.game.Data.StringListObj[stringListById].Data[self.currentHelp, 2];
      let mut index1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].Data[self.currentHelp, 3]));
      str2: String = self.game.Data.StringListObj[stringListById].Data[self.currentHelp, 4];
      if (index1 > -1)
        index1 = self.game.Data.EventPicNr[index1];
      TextAreaClass2 textAreaClass2 = new TextAreaClass2(self.game, self.w - 190, 17, self.game.MarcFont8, "\r\n\r\n\r\n" + str1, 17,  self.OwnBitmap, 165, -15, true, tUseEncy: true);
      textAreaClass2.Paint();
      DrawMod.Draw( g,  textAreaClass2.OwnBitmap, 165, -15, 0.0f, 0.0f, 0.0f, 1f);
      let mut num: i32 = textAreaClass2.HeightUsed();
      textAreaClass2.Dispose();
      DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont2, 179, 19, Color.White);
      DrawMod.DrawBlock( g, 179, 49, 450, 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      if (index1 <= -1)
        return;
      let mut tsubpart: SubPartClass =  ButtonPartClass::new(index1, tDescript: "Click to start video", tVideoMode: true);
      self.Info1Id = self.AddSubPart( tsubpart, 179, num + 24, BitmapStore.GetWidth(index1), BitmapStore.Getheight(index1), 1);
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
          if (self.SubPartID[index] == self.Info1Id)
          {
            self.game.EditObj.TipTitle = "PLAY VIDEO";
            self.game.EditObj.TipText = "Will open a video file in an external video player. You can use alt-TAB to switch between game and video player.";
          }
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
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (self.MouseData[index] > 0 && x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] >= 1000)
          {
            self.currentHelp = self.MouseData[index] - 1000;
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
          if (self.MouseData[index] == 999)
          {
            self.game.EditObj.SetViewMode2 = 0;
            windowReturnClass1.AddCommand(1, 9);
            windowReturnClass1.AddCommand(7, 12);
            windowReturnClass1.AddCommand(4, 67);
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
        }
      }
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index] && self.SubPartID[index] == self.Info1Id)
        {
          str: String = self.game.Data.StringListObj[self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[988]))].Data[self.currentHelp, 4];
          self.SubPartFlag[index] = true;
          windowReturnClass1.SetFlag(true);
          windowReturnClass2: WindowReturnClass;
          try
          {
            Process.Start(AppDomain.CurrentDomain.BaseDirectory + str);
            self.game.FormRef.SendToBack();
            SoundMod.STopEventBackground();
            SoundMod.STopEventWave();
            SoundMod.NOSOUND = true;
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            let mut num: i32 =  Interaction.MsgBox( "PROBLEM", Title: ( "Sadly there was a problem trying to let your Windows system run this video. Please check the game forums for possible causes."));
            self.dostuff();
            windowReturnClass2 = windowReturnClass1;
            ProjectData.ClearProjectError();
            goto label_19;
          }
          self.dostuff();
          return windowReturnClass1;
label_19:
          return windowReturnClass2;
        }
      }
      if (x > 8 & x < self.w - 8 && y < self.h - 24)
        windowReturnClass1.NoMouseClickBelow = true;
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }
  }
}
