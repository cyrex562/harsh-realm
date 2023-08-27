// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimplePrefsWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SimplePrefsWindowClass : WindowClass
  {
     cancelid: i32;
     Info1Id: i32;
     info2id: i32;
     ShowString: String;
     DateTime ShowTime;
     w: i32;
     h: i32;
     CurrentView: i32;
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
     B16Id: i32;
     B16TextId: i32;
     slider1id: i32;
     slider2id: i32;

    pub SimplePrefsWindowClass( tGame: GameClass)
      : base( tGame, 480, 250, 8)
    {
      self.View();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            self.SubPartList[index].DescriptInfo(x - self.SubPartX[index], y - self.SubPartY[index]);
            if (Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
            {
              self.game.EditObj.TipButton = true;
              self.game.EditObj.TipTitle = "";
              self.game.EditObj.TipText = self.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 0)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          break;
        }
      }
    }

    pub fn View()
    {
      if (self.cancelid > 0)
      {
        self.RemoveSubPart(self.cancelid);
        self.cancelid = 0;
      }
      if (self.B1Id > 0)
        self.RemoveSubPart(self.B1Id);
      if (self.B1TextId > 0)
        self.RemoveSubPart(self.B1TextId);
      if (self.B2Id > 0)
        self.RemoveSubPart(self.B2Id);
      if (self.B2TextId > 0)
        self.RemoveSubPart(self.B2TextId);
      if (self.B3Id > 0)
        self.RemoveSubPart(self.B3Id);
      if (self.B3TextId > 0)
        self.RemoveSubPart(self.B3TextId);
      if (self.B4Id > 0)
        self.RemoveSubPart(self.B4Id);
      if (self.B4TextId > 0)
        self.RemoveSubPart(self.B4TextId);
      if (self.B5Id > 0)
        self.RemoveSubPart(self.B5Id);
      if (self.B5TextId > 0)
        self.RemoveSubPart(self.B5TextId);
      if (self.B6Id > 0)
        self.RemoveSubPart(self.B6Id);
      if (self.B6TextId > 0)
        self.RemoveSubPart(self.B6TextId);
      if (self.B7Id > 0)
        self.RemoveSubPart(self.B7Id);
      if (self.B7TextId > 0)
        self.RemoveSubPart(self.B7TextId);
      if (self.B8Id > 0)
        self.RemoveSubPart(self.B8Id);
      if (self.B8TextId > 0)
        self.RemoveSubPart(self.B8TextId);
      if (self.B9Id > 0)
        self.RemoveSubPart(self.B9Id);
      if (self.B9TextId > 0)
        self.RemoveSubPart(self.B9TextId);
      if (self.B10Id > 0)
        self.RemoveSubPart(self.B10Id);
      if (self.B10TextId > 0)
        self.RemoveSubPart(self.B10TextId);
      if (self.B11Id > 0)
        self.RemoveSubPart(self.B11Id);
      if (self.B11TextId > 0)
        self.RemoveSubPart(self.B11TextId);
      if (self.B12Id > 0)
        self.RemoveSubPart(self.B12Id);
      if (self.B12TextId > 0)
        self.RemoveSubPart(self.B12TextId);
      if (self.B13Id > 0)
        self.RemoveSubPart(self.B13Id);
      if (self.B13TextId > 0)
        self.RemoveSubPart(self.B13TextId);
      if (self.B14Id > 0)
        self.RemoveSubPart(self.B14Id);
      if (self.B14TextId > 0)
        self.RemoveSubPart(self.B14TextId);
      if (self.B15Id > 0)
        self.RemoveSubPart(self.B15Id);
      if (self.B15TextId > 0)
        self.RemoveSubPart(self.B15TextId);
      if (self.B16Id > 0)
        self.RemoveSubPart(self.B16Id);
      if (self.B16TextId > 0)
        self.RemoveSubPart(self.B16TextId);
      if (self.slider1id > 0)
      {
        self.RemoveSubPart(self.slider1id);
        self.slider1id = 0;
      }
      if (self.slider2id > 0)
      {
        self.RemoveSubPart(self.slider2id);
        self.slider2id = 0;
      }
      self.ClearMouse();
      self.NewBackGroundAndClearAll(480, 250, -1);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.HighQuality;
      graphics.InterpolationMode = InterpolationMode.HighQualityBicubic;
      DrawMod.DrawMessFrame( self.OwnBitmap,  graphics, 0, 0, 480, 250);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      if (!self.game.EditObj.SoundOn)
      {
        let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "Sound effects are currently turned off.",  self.OwnBitmap, 40, 110);
        self.B1Id = self.AddSubPart( tsubpart, 40, 110, 35, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "Sound effects are  currently turned on.",  self.OwnBitmap, 40, 110);
        self.B1Id = self.AddSubPart( tsubpart, 40, 110, 35, 35, 1);
      }
      DrawMod.DrawTextColouredMarc( graphics, "SOUND FX", self.game.MarcFont4, 90, 118, Color.White);
      if (!self.game.EditObj.IntroSoundOn)
      {
        let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "Music is currently turned off.",  self.OwnBitmap, 40, 40);
        self.B8Id = self.AddSubPart( tsubpart, 40, 40, 35, 35, 1);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "Music is currently turned on.",  self.OwnBitmap, 40, 40);
        self.B8Id = self.AddSubPart( tsubpart, 40, 40, 35, 35, 1);
      }
      DrawMod.DrawTextColouredMarc( graphics, "MUSIC", self.game.MarcFont4, 90, 48, Color.White);
      SubPartClass tsubpart1;
      if (self.slider1id < 1)
      {
        tsubpart1 =  new NumberSliderSubPartClass2(self.game, "Music Volume = ", "%", 200, 0, 100, self.game.EditObj.Volume, tbackbitmap: ( self.OwnBitmap), bbx: 230, bby: 30, tMarc: true);
        self.slider1id = self.AddSubPart( tsubpart1, 230, 30, 200, 40, 0);
      }
      if (self.slider2id < 1)
      {
        tsubpart1 =  new NumberSliderSubPartClass2(self.game, "SFX Volume = ", "%", 200, 0, 100, self.game.EditObj.Volume2, tbackbitmap: ( self.OwnBitmap), bbx: 230, bby: 100, tMarc: true);
        self.slider2id = self.AddSubPart( tsubpart1, 230, 100, 200, 40, 0);
      }
      tsubpart1 =  new TextButtonPartClass("OK", 150, "Click to return to main screen. [ESC]",  self.OwnBitmap, 165, 165, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.cancelid = self.AddSubPart( tsubpart1, 165, 165, 150, 40, 1);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          windowReturnClass.AddCommand(6, 0);
          self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
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
            let mut num: i32 = self.SubPartID[index];
            if (num == self.cancelid)
            {
              windowReturnClass.AddCommand(6, 0);
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.slider1id)
            {
              self.game.EditObj.Volume = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              self.SubPartList[index].Scroller = true;
              SoundMod.ChangeEventSoundBg(self.game.EditObj);
              return windowReturnClass;
            }
            if (num == self.slider2id)
            {
              self.game.EditObj.Volume2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              self.SubPartList[index].Scroller = true;
              SoundMod.ChangeEventSound(self.game.EditObj);
              return windowReturnClass;
            }
            if (num == self.B1Id)
            {
              self.game.EditObj.SoundOn = !self.game.EditObj.SoundOn;
              self.View();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.B8Id)
            {
              self.game.EditObj.IntroSoundOn = !self.game.EditObj.IntroSoundOn;
              if (self.game.EditObj.IntroSoundOn)
                SoundMod.PlayEventBackground(self.game.AppPath + "sound/" + self.game.ModOpeningSoundtrack,  self.game.EditObj);
              if (!self.game.EditObj.IntroSoundOn)
                SoundMod.STopEventBackground();
              Application.DoEvents();
              self.View();
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

    pub HandleMouseUp: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult = OrderResult::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (self.SubPartList[index].Scroller)
          {
            let mut num: i32 = self.SubPartID[index];
            if (num == self.slider1id)
            {
              self.game.EditObj.Volume = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              self.SubPartList[index].Scroller = false;
              self.SubPartList[self.SubpartNr(self.slider2id)].Scroller = false;
              SoundMod.ChangeEventSoundBg(self.game.EditObj);
              return windowReturnClass;
            }
            if (num == self.slider2id)
            {
              self.game.EditObj.Volume2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              self.SubPartList[index].Scroller = false;
              self.SubPartList[self.SubpartNr(self.slider1id)].Scroller = false;
              SoundMod.ChangeEventSound(self.game.EditObj);
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
  }
}
