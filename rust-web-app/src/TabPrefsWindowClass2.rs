// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TabPrefsWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Runtime.InteropServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class TabPrefsWindowClass2 : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     ShowString: String;
     DateTime ShowTime;
     w: i32;
     h: i32;
     CurrentView: i32;
     currentCat: i32;
     BNameId: i32;
     BNameTextId: i32;
     B1Id: i32;
     B1TextId: i32;
     saveid: i32;
     quitid: i32;
     minId: i32;
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
     B17Id: i32;
     B17TextId: i32;
     B17bId: i32;
     B17bTextId: i32;
     B17cId: i32;
     B17cTextId: i32;
     B17dId: i32;
     B17dTextId: i32;
     B17eId: i32;
     B17eTextId: i32;
     B17fId: i32;
     B17fTextId: i32;
     B18Id: i32;
     B18TextId: i32;
     B19Id: i32;
     B19TextId: i32;
     B20Id: i32;
     B20TextId: i32;
     B21Id: i32;
     B21TextId: i32;
     B22Id: i32;
     B22TextId: i32;
     B23Id: i32;
     B23TextId: i32;
     B24Id: i32;
     B24TextId: i32;
     b25id: i32;
     b25Textid: i32;
     b26id: i32;
     b26Textid: i32;
     b27id: i32;
     b27Textid: i32;
     b28id: i32;
     b28Textid: i32;
     b29id: i32;
     b29Textid: i32;
     b30id: i32;
     b30Textid: i32;
     b31id: i32;
     b31Textid: i32;
     b32id: i32;
     b32Textid: i32;
     b33id: i32;
     b33Textid: i32;
     b34id: i32;
     b34Textid: i32;
     b35id: i32;
     b35Textid: i32;
     b36id: i32;
     b36Textid: i32;
     slider1id: i32;
     slider2id: i32;
     ListClass OptionsListObj;
     optionsListId: i32;
     detailnr: i32;
     SimpleList resList;

    pub TabPrefsWindowClass2(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect,
      Rectangle trect)
      : base( tGame, trect.Width, trect.Height, 8)
    {
      self.detailnr = -1;
      self.w = trect.Width;
      self.h = trect.Height;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.currentCat = 1;
      self.dostuff();
      self.detailnr = -1;
    }

    pub fn DoRefresh() => self.dostuff();

    pub fn dostuff()
    {
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
      if (self.B17Id > 0)
        self.RemoveSubPart(self.B17Id);
      if (self.B17TextId > 0)
        self.RemoveSubPart(self.B17TextId);
      if (self.B17bId > 0)
        self.RemoveSubPart(self.B17bId);
      if (self.B17bTextId > 0)
        self.RemoveSubPart(self.B17bTextId);
      if (self.B17cId > 0)
        self.RemoveSubPart(self.B17cId);
      if (self.B17cTextId > 0)
        self.RemoveSubPart(self.B17cTextId);
      if (self.B17dId > 0)
        self.RemoveSubPart(self.B17dId);
      if (self.B17dTextId > 0)
        self.RemoveSubPart(self.B17dTextId);
      if (self.B17eId > 0)
        self.RemoveSubPart(self.B17eId);
      if (self.B17eTextId > 0)
        self.RemoveSubPart(self.B17eTextId);
      if (self.B17fId > 0)
        self.RemoveSubPart(self.B17fId);
      if (self.B17fTextId > 0)
        self.RemoveSubPart(self.B17fTextId);
      if (self.B18Id > 0)
        self.RemoveSubPart(self.B18Id);
      if (self.B18TextId > 0)
        self.RemoveSubPart(self.B18TextId);
      if (self.B19Id > 0)
        self.RemoveSubPart(self.B19Id);
      if (self.B19TextId > 0)
        self.RemoveSubPart(self.B19TextId);
      if (self.B20Id > 0)
        self.RemoveSubPart(self.B20Id);
      if (self.B20TextId > 0)
        self.RemoveSubPart(self.B20TextId);
      if (self.B21Id > 0)
        self.RemoveSubPart(self.B21Id);
      if (self.B21TextId > 0)
        self.RemoveSubPart(self.B21TextId);
      if (self.B22Id > 0)
        self.RemoveSubPart(self.B22Id);
      if (self.B22TextId > 0)
        self.RemoveSubPart(self.B22TextId);
      if (self.B23Id > 0)
        self.RemoveSubPart(self.B23Id);
      if (self.B23TextId > 0)
        self.RemoveSubPart(self.B23TextId);
      if (self.B24Id > 0)
        self.RemoveSubPart(self.B24Id);
      if (self.B24TextId > 0)
        self.RemoveSubPart(self.B24TextId);
      if (self.b25id > 0)
        self.RemoveSubPart(self.b25id);
      if (self.b25Textid > 0)
        self.RemoveSubPart(self.b25Textid);
      if (self.b26id > 0)
        self.RemoveSubPart(self.b26id);
      if (self.b26Textid > 0)
        self.RemoveSubPart(self.b26Textid);
      if (self.b27id > 0)
        self.RemoveSubPart(self.b27id);
      if (self.b27Textid > 0)
        self.RemoveSubPart(self.b27Textid);
      if (self.b28id > 0)
        self.RemoveSubPart(self.b28id);
      if (self.b28Textid > 0)
        self.RemoveSubPart(self.b28Textid);
      if (self.b29id > 0)
        self.RemoveSubPart(self.b29id);
      if (self.b30id > 0)
        self.RemoveSubPart(self.b30id);
      if (self.b30Textid > 0)
        self.RemoveSubPart(self.b30Textid);
      if (self.b31id > 0)
        self.RemoveSubPart(self.b31id);
      if (self.b31Textid > 0)
        self.RemoveSubPart(self.b31Textid);
      if (self.b32id > 0)
        self.RemoveSubPart(self.b32id);
      if (self.b32Textid > 0)
        self.RemoveSubPart(self.b32Textid);
      if (self.b33id > 0)
        self.RemoveSubPart(self.b33id);
      if (self.b33Textid > 0)
        self.RemoveSubPart(self.b33Textid);
      if (self.b34id > 0)
        self.RemoveSubPart(self.b34id);
      if (self.b34Textid > 0)
        self.RemoveSubPart(self.b34Textid);
      if (self.b35id > 0)
        self.RemoveSubPart(self.b35id);
      if (self.b35Textid > 0)
        self.RemoveSubPart(self.b35Textid);
      if (self.b36id > 0)
        self.RemoveSubPart(self.b36id);
      if (self.b36Textid > 0)
        self.RemoveSubPart(self.b36Textid);
      if (self.minId > 0)
        self.RemoveSubPart(self.minId);
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
      if (self.currentCat != 4 && self.optionsListId > 0)
      {
        self.RemoveSubPart(self.optionsListId);
        self.optionsListId = 0;
      }
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      Rectangle trect1 = DrawMod.DrawBackTab(g, self.w, self.h, "PREFS", 8);
      self.AddMouse( trect1, "CLOSE TAB", "Click here to close this tab. [ESC/F1]", 999);
      if (self.currentCat < 1)
        self.currentCat = 1;
      g.SmoothingMode = SmoothingMode.None;
      DrawMod.drawLineDot( g, 135, 0, 135, 240, Color.White);
      DrawMod.drawLineDot( g, 15, 20, 135, 20, Color.White);
      let mut num1: i32 = 1;
      do
      {
        g.SmoothingMode = SmoothingMode.AntiAlias;
        let mut num2: i32 = num1;
        if (num2 > 6)
          num2 = 6;
        if (num2 > 0 & self.currentCat < 1)
          self.currentCat = num1;
        if (num1 == self.currentCat)
          DrawMod.DrawBlockGradient( g, 35, 40 * num1 - 20, 100, 40, Color.FromArgb(0, 0, 0, 0), Color.FromArgb(96,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
        tstring: String;
        if (num1 == 1)
          tstring = "General";
        if (num1 == 2)
          tstring = "Map";
        if (num1 == 3)
          tstring = "Sound";
        if (num1 == 4)
          tstring = "Gfx & Resolution";
        if (num1 == 5)
          tstring = "AI Speed";
        Rectangle trect2 = Rectangle::new(15, 40 * num1 - 20, 120, 40);
        self.AddMouse( trect2, "Preferences Category " + tstring, "Click to see all all preference settings in this category", 1000 + num1);
        let mut y: i32 = 40 * num1 - 20 + 13;
        DrawMod.DrawTextColouredMarcCenter( g, tstring, self.game.MarcFont5, 75, y, Color.White);
        g.SmoothingMode = SmoothingMode.None;
        DrawMod.drawLineDot( g, 15, 20 + num1 * 40, 135, 20 + num1 * 40, Color.White);
        num1 += 1;
      }
      while (num1 <= 5);
      g.SmoothingMode = SmoothingMode.AntiAlias;
      let mut num3: i32 = 200;
      let mut num4: i32 = 25;
      let mut num5: i32 = 45;
      let mut num6: i32 = 220;
      let mut num7: i32 = num4;
      if (self.currentCat == 5)
      {
        let mut num8: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 156, 0, 0))].GetData(0, 56, 2)));
        if (num8 != 0)
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "The time the AI takes to make it moves.",  self.OwnBitmap, num3, num4);
          self.b35id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "The time the AI takes to make it moves.",  self.OwnBitmap, num3, num4);
          self.b35id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "NORMAL AI PROCESSING - Level 0", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        let mut num9: i32 = num4 + num5;
        if (num8 != 1)
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "The time the AI takes to make it moves.",  self.OwnBitmap, num3, num9);
          self.b33id = self.AddSubPart( tsubpart, num3, num9, 35, 35, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "The time the AI takes to make it moves.",  self.OwnBitmap, num3, num9);
          self.b33id = self.AddSubPart( tsubpart, num3, num9, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "SLOW AI PROCESSING - Level 100", self.game.MarcFont4, num3 + 50, num9 + 8, Color.White);
        let mut num10: i32 = num9 + num5;
        if (num8 != 2)
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "The time the AI takes to make it moves.",  self.OwnBitmap, num3, num10);
          self.b34id = self.AddSubPart( tsubpart, num3, num10, 35, 35, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "The time the AI takes to make it moves.",  self.OwnBitmap, num3, num10);
          self.b34id = self.AddSubPart( tsubpart, num3, num10, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "VERY SLOW AI PROCESSING - Level 250", self.game.MarcFont4, num3 + 50, num10 + 8, Color.White);
        num4 = num10 + num5;
      }
      num11: i32;
      if (self.currentCat == 3)
      {
        if (!self.game.EditObj.IntroSoundOn)
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "Music is currently turned off.",  self.OwnBitmap, num3, num4);
          self.B8Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "Music is currently turned on.",  self.OwnBitmap, num3, num4);
          self.B8Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "MUSIC", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        let mut num12: i32 = num4 + num5;
        if ( self.game.Data.RuleVar[990] > 0.0 & self.game.Data.TempString[730].Length > 0)
        {
          if (!self.game.EditObj.AlternateMusic)
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "Click to switch to alternate music configuration. ",  self.OwnBitmap, num3, num12);
            self.B19Id = self.AddSubPart( tsubpart, num3, num12, 35, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "Click to switch to defeault music configuration. ",  self.OwnBitmap, num3, num12);
            self.B19Id = self.AddSubPart( tsubpart, num3, num12, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, self.game.Data.TempString[730].ToUpper(), self.game.MarcFont4, num3 + 50, num12 + 8, Color.White);
        }
        let mut num13: i32 = num12 + num5;
        if (!self.game.EditObj.SoundOn)
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "Sound effects are currently turned off.",  self.OwnBitmap, num3, num13);
          self.B1Id = self.AddSubPart( tsubpart, num3, num13, 35, 35, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "Sound effects are  currently turned on.",  self.OwnBitmap, num3, num13);
          self.B1Id = self.AddSubPart( tsubpart, num3, num13, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "SOUND FX", self.game.MarcFont4, num3 + 50, num13 + 8, Color.White);
        num11 = num13 + num5;
        let mut num14: i32 = num7 - 5;
        num3 += num6;
        if (self.slider1id < 1)
        {
          let mut tsubpart: SubPartClass =  new NumberSliderSubPartClass2(self.game, "Music Volume = ", "%", 200, 0, 100, self.game.EditObj.Volume, tbackbitmap: ( self.OwnBitmap), bbx: num3, bby: num14, tMarc: true);
          self.slider1id = self.AddSubPart( tsubpart, num3, num14, 200, 40, 0);
        }
        num4 = num14 + (num5 + num5 - 5);
        if (self.slider2id < 1)
        {
          let mut tsubpart: SubPartClass =  new NumberSliderSubPartClass2(self.game, "SFX Volume = ", "%", 200, 0, 100, self.game.EditObj.Volume2, tbackbitmap: ( self.OwnBitmap), bbx: num3, bby: num4, tMarc: true);
          self.slider2id = self.AddSubPart( tsubpart, num3, num4, 200, 40, 0);
        }
      }
      if (self.currentCat == 2)
      {
        if (!self.game.EditObj.PrefShowFOW)
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "Either hide or show the FOW area with a dark shrowd.",  self.OwnBitmap, num3, num4);
          self.B2Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
        }
        else
        {
          let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "Either hide or show the FOW area with a dark shrowd.",  self.OwnBitmap, num3, num4);
          self.B2Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "SHOW FOW", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        let mut num15: i32 = num4 + num5;
        if (self.game.Data.Product != 7)
        {
          if (!self.game.EditObj.HexRasterOn)
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "Either hide or show the hex grid.",  self.OwnBitmap, num3, num15);
            self.B4Id = self.AddSubPart( tsubpart, num3, num15, 35, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "Either hide or show the hex grid.",  self.OwnBitmap, num3, num15);
            self.B4Id = self.AddSubPart( tsubpart, num3, num15, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "HEX GRID", self.game.MarcFont4, num3 + 50, num15 + 8, Color.White);
          num15 += num5;
        }
        if (self.game.Data.Product != 7)
        {
          if (!self.game.EditObj.ShowLabel)
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "When option is not active text labels will be hidden.",  self.OwnBitmap, num3, num15);
            self.B10Id = self.AddSubPart( tsubpart, num3, num15, 35, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "When option is not active text labels will be hidden.",  self.OwnBitmap, num3, num15);
            self.B10Id = self.AddSubPart( tsubpart, num3, num15, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "SHOW LABEL", self.game.MarcFont4, num3 + 50, num15 + 8, Color.White);
          num15 += num5;
        }
        if (self.game.Data.Product != 7)
        {
          if (!self.game.EditObj.RegimeColoring)
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "When option is active hexes will be colored\r\non a regime-to-regime basis.",  self.OwnBitmap, num3, num15);
            self.B16Id = self.AddSubPart( tsubpart, num3, num15, 35, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "When option is active hexes will be colored\r\non a regime-to-regime basis.",  self.OwnBitmap, num3, num15);
            self.B16Id = self.AddSubPart( tsubpart, num3, num15, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "REGIME COLORING", self.game.MarcFont4, num3 + 50, num15 + 8, Color.White);
          num15 += num5;
        }
        if ( self.game.Data.RuleVar[408] > 0.0)
        {
          if (self.game.EditObj.HideUnit == 2)
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "You can toggle 'nato style' counters on/off here. ",  self.OwnBitmap, num3, num15);
            self.B21Id = self.AddSubPart( tsubpart, num3, num15, 35, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "You can toggle 'nato style' counters on/off here. ",  self.OwnBitmap, num3, num15);
            self.B21Id = self.AddSubPart( tsubpart, num3, num15, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "NATO counters", self.game.MarcFont4, num3 + 50, num15 + 8, Color.White);
          let mut num16: i32 = num15 + num5;
          if (self.game.EditObj.SpreadUnit)
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "You can toggle 'having multiple counters on same hex in full zoom in mode' on/off here. ",  self.OwnBitmap, num3, num16);
            self.B22Id = self.AddSubPart( tsubpart, num3, num16, 35, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "You can toggle 'having multiple counters on same hex in full zoom in mode' counters on/off here. ",  self.OwnBitmap, num3, num16);
            self.B22Id = self.AddSubPart( tsubpart, num3, num16, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "Smaller counters", self.game.MarcFont4, num3 + 50, num16 + 8, Color.White);
          num11 = num16 + num5;
        }
        num4 = num7;
        num3 += num6;
        if (self.game.Data.Product <= 6)
        {
          if (!self.game.EditObj.PrefMinimalistCounter)
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "When option is active minimalist counter graphics will be shown. ",  self.OwnBitmap, num3, num4);
            self.B15Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "When option is active minimalist counter graphics will be shown.",  self.OwnBitmap, num3, num4);
            self.B15Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "MINIMALIST COUNTERS", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
          num4 += num5;
        }
        if (self.game.Data.Product != 7)
        {
          if (!self.game.EditObj.ShowHQPower)
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "When option is active HQ Power will be shown on map\r\nCan slow down performance. Hotkey: 5",  self.OwnBitmap, num3, num4);
            self.B12Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "When option is active HQ Power will be shown on map\r\nCan slow down performance. Hotkey: 5",  self.OwnBitmap, num3, num4);
            self.B12Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "HQ POWER RANGE", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
          num4 += num5;
        }
        if (self.game.Data.Product != 7)
        {
          if (!self.game.EditObj.ShowUnderHQ)
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "When option active subordinate units will be highlighted when you select a HQ.",  self.OwnBitmap, num3, num4);
            self.B3Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
          }
          else
          {
            let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "When option active subordinate units will be highlighted when you select a HQ.",  self.OwnBitmap, num3, num4);
            self.B3Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "HQ SUBORDINATES", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
          num4 += num5;
        }
        if (self.game.Data.Product != 7)
        {
          if ( self.game.Data.RuleVar[976] == 1.0)
          {
            if (!self.game.EditObj.ShowSameHistorical)
            {
              let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "When option active non-HQ units belonging to same HQ will be highlighted.",  self.OwnBitmap, num3, num4);
              self.B6Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
            }
            else
            {
              let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "When option active non-HQ units belonging to same HQ will be highlighted.",  self.OwnBitmap, num3, num4);
              self.B6Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
            }
            DrawMod.DrawTextColouredMarc( g, "SHOW DIVISIONS", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
            num4 += num5;
          }
          else
          {
            if (!self.game.EditObj.ShowSameHistorical)
            {
              let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "When option active units belonging to same division will be highlighted.",  self.OwnBitmap, num3, num4);
              self.B6Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
            }
            else
            {
              let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "When option active units belonging to same division will be highlighted.",  self.OwnBitmap, num3, num4);
              self.B6Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
            }
            DrawMod.DrawTextColouredMarc( g, "SHOW DIVISIONS", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
            num4 += num5;
          }
        }
        if (self.game.Data.Product != 7)
        {
          if ( self.game.Data.RuleVar[419] > 0.0)
          {
            if (!self.game.EditObj.ShowAirRange)
            {
              let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "When option is active the LOS range of the Unit is shown. Hotkey: 6 ",  self.OwnBitmap, num3, num4);
              self.B14Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
            }
            else
            {
              let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "When option is active the LOS range of the Unit is shown. Hotkey: 6 ",  self.OwnBitmap, num3, num4);
              self.B14Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
            }
            DrawMod.DrawTextColouredMarc( g, "LINE OF SIGHT", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
            num4 += num5;
          }
          else if ( self.game.Data.RuleVar[976] < 1.0 &&  self.game.Data.RuleVar[990] < 1.0)
          {
            if (!self.game.EditObj.ShowAirRange)
            {
              let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, false, "When option is active air intercept range will be shown on map when selecting air unit. \r\nAnd showing artillery range when selecting artillery unit. \r\nCan slow down performance. Hotkey: 6 ",  self.OwnBitmap, num3, num4);
              self.B14Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
            }
            else
            {
              let mut tsubpart: SubPartClass =  new MarcRadioPartClass(0, true, "When option is active air intercept range will be shown on map when selecting air unit. \r\nAnd showing artillery range when selecting artillery unit. \r\nCan slow down performance. Hotkey: 6 ",  self.OwnBitmap, num3, num4);
              self.B14Id = self.AddSubPart( tsubpart, num3, num4, 35, 35, 1);
            }
            DrawMod.DrawTextColouredMarc( g, "AIR + ART RANGE", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
            num4 += num5;
          }
        }
      }
      SubPartClass tsubpart1;
      if (self.currentCat == 1)
      {
        if (!self.game.EditObj.AutoSave)
        {
          let mut tsubpart2: SubPartClass =  new MarcRadioPartClass(0, false, "Auto-saves are made moment you press end turn button.",  self.OwnBitmap, num3, num4);
          self.B9Id = self.AddSubPart( tsubpart2, num3, num4, 35, 35, 1);
        }
        else
        {
          let mut tsubpart3: SubPartClass =  new MarcRadioPartClass(0, true, "Auto-saves are made moment you press end turn button.",  self.OwnBitmap, num3, num4);
          self.B9Id = self.AddSubPart( tsubpart3, num3, num4, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "AUTO SAVE", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        let mut num17: i32 = num4 + num5;
        if (!self.game.EditObj.Screenshoton)
        {
          let mut tsubpart4: SubPartClass =  new MarcRadioPartClass(0, false, "At start and end of turn automatic screenshots can enabled.\r\nLook in screenshots directory to find them.",  self.OwnBitmap, num3, num17);
          self.B13Id = self.AddSubPart( tsubpart4, num3, num17, 35, 35, 1);
        }
        else
        {
          let mut tsubpart5: SubPartClass =  new MarcRadioPartClass(0, true, "At start and end of turn automatic screenshots can enabled.\r\nLook in screenshots directory to find them.",  self.OwnBitmap, num3, num17);
          self.B13Id = self.AddSubPart( tsubpart5, num3, num17, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "SCREENSHOTS", self.game.MarcFont4, num3 + 50, num17 + 8, Color.White);
        let mut num18: i32 = num17 + num5;
        if (self.game.Data.Product != 7)
        {
          if (!self.game.EditObj.CombatNumbers)
          {
            let mut tsubpart6: SubPartClass =  new MarcRadioPartClass(0, false, "Change the way combat results are presented",  self.OwnBitmap, num3, num18);
            self.B5Id = self.AddSubPart( tsubpart6, num3, num18, 35, 35, 1);
          }
          else
          {
            let mut tsubpart7: SubPartClass =  new MarcRadioPartClass(0, true, "Change the way combat results are presented",  self.OwnBitmap, num3, num18);
            self.B5Id = self.AddSubPart( tsubpart7, num3, num18, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "COMBAT NUMBERS", self.game.MarcFont4, num3 + 50, num18 + 8, Color.White);
          num18 += num5;
        }
        if (!self.game.EditObj.ShowMouseOver)
        {
          let mut tsubpart8: SubPartClass =  new MarcRadioPartClass(0, false, "When option is not active mouse overs will only appear on right click.",  self.OwnBitmap, num3, num18);
          self.B7Id = self.AddSubPart( tsubpart8, num3, num18, 35, 35, 1);
        }
        else
        {
          let mut tsubpart9: SubPartClass =  new MarcRadioPartClass(0, true, "When option is not active mouse overs will only appear on right click.",  self.OwnBitmap, num3, num18);
          self.B7Id = self.AddSubPart( tsubpart9, num3, num18, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "AUTO MOUSE OVER", self.game.MarcFont4, num3 + 50, num18 + 8, Color.White);
        let mut num19: i32 = num18 + num5;
        if (self.game.Data.Product >= 6)
        {
          if (!self.game.EditObj.AutoCombat)
          {
            let mut tsubpart10: SubPartClass =  new MarcRadioPartClass(0, false, "When option is not active combat will not automaticly commence resolution.",  self.OwnBitmap, num3, num19);
            self.b26id = self.AddSubPart( tsubpart10, num3, num19, 35, 35, 1);
          }
          else
          {
            let mut tsubpart11: SubPartClass =  new MarcRadioPartClass(0, true, "When option is not active combat will not automaticly commence resolution.",  self.OwnBitmap, num3, num19);
            self.b26id = self.AddSubPart( tsubpart11, num3, num19, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "AUTO COMBAT", self.game.MarcFont4, num3 + 50, num19 + 8, Color.White);
          num19 += num5;
        }
        if (self.game.Data.Product == 7)
        {
          if (!self.game.EditObj.maximumInterfaceSpace)
          {
            let mut tsubpart12: SubPartClass =  new MarcRadioPartClass(0, false, "When option is not active Troops and Asset display space in the bottom will be slightly reduced.",  self.OwnBitmap, num3, num19);
            self.b27id = self.AddSubPart( tsubpart12, num3, num19, 35, 35, 1);
          }
          else
          {
            let mut tsubpart13: SubPartClass =  new MarcRadioPartClass(0, true, "When option is active Troops and Asset display space in the bottom will be maximalized.",  self.OwnBitmap, num3, num19);
            self.b27id = self.AddSubPart( tsubpart13, num3, num19, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "MAX ASSET SPACE", self.game.MarcFont4, num3 + 50, num19 + 8, Color.White);
          num11 = num19 + num5;
        }
        let mut num20: i32 = num7;
        num3 += num6;
        if (self.game.Data.Product != 7)
        {
          if (!self.game.EditObj.useLeftRightClickMode)
          {
            let mut tsubpart14: SubPartClass =  new MarcRadioPartClass(0, false, "When option is active you'll use the new left/right click interface for moving.",  self.OwnBitmap, num3, num20);
            self.b28id = self.AddSubPart( tsubpart14, num3, num20, 35, 35, 1);
          }
          else
          {
            let mut tsubpart15: SubPartClass =  new MarcRadioPartClass(0, true, "When option is active you'll use the new left/right click interface for moving.",  self.OwnBitmap, num3, num20);
            self.b28id = self.AddSubPart( tsubpart15, num3, num20, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "LEFT/RIGHT CLICK INTERFACE", self.game.MarcFont4, num3 + 50, num20 + 8, Color.White);
          num20 += num5;
        }
        if ( self.game.Data.RuleVar[408] > 0.0)
        {
          if (!self.game.Data.PBEM | self.game.SuperAdminRights)
          {
            tsubpart1 =  new MarcButtonPartClass(self.game.BUTTONSAVE, tDescript: "Save", tBackbitmap: ( self.OwnBitmap), bbx: num3, bby: num20);
            self.B23Id = self.AddSubPart( tsubpart1, num3, num20, 35, 35, 1);
            DrawMod.DrawTextColouredMarc( g, "SAVE GAME", self.game.MarcFont4, num3 + 50, num20 + 8, Color.White);
            num20 += num5;
          }
          tsubpart1 =  new MarcButtonPartClass(self.game.BUTTONQUIT, tDescript: "Quit", tBackbitmap: ( self.OwnBitmap), bbx: num3, bby: num20);
          self.b25id = self.AddSubPart( tsubpart1, num3, num20, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "QUIT GAME", self.game.MarcFont4, num3 + 50, num20 + 8, Color.White);
          num20 += num5;
          tsubpart1 =  new MarcButtonPartClass(self.game.SYSTEM1, tDescript: "Minimize", tBackbitmap: ( self.OwnBitmap), bbx: num3, bby: num20);
          self.minId = self.AddSubPart( tsubpart1, num3, num20, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "MINIMIZE GAME", self.game.MarcFont4, num3 + 50, num20 + 8, Color.White);
        }
        let mut num21: i32 = num20 - num5 * 2;
        if (!self.game.EditObj.allowMetrics)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active some (1<KB) game progress core data will at times be sent to " + self.game.MetricsURL + " to help us improve the game balance. You will remain anonymous. It will be appreciated if you switch this on since balancing requires lots of player feedback. ",  self.OwnBitmap, num3 + 170, num21);
          self.B18Id = self.AddSubPart( tsubpart1, num3 + 170, num21, 35, 35, 1);
        }
        else
        {
          tsubpart1 =  new MarcRadioPartClass(0, true, "When option is active some (1<KB) game progress core data will at times be sent to " + self.game.MetricsURL + " to help us improve the game balance. You will remain anonymous. It will be appreciated if you switch this on since balancing requires lots of player feedback. ",  self.OwnBitmap, num3 + 170, num21);
          self.B18Id = self.AddSubPart( tsubpart1, num3 + 170, num21, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "SHARE METRICS", self.game.MarcFont4, num3 + 170 + 50, num21 + 8, Color.White);
        let mut num22: i32 = num21 + num5 * 3;
        self.game.Data.DontShowAIMove = self.game.EditObj.dontShowAImoves;
        if (self.game.EditObj.dontShowAImoves)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active you'll be in History Screen while AI makes moves.",  self.OwnBitmap, num3, num22);
          self.b30id = self.AddSubPart( tsubpart1, num3, num22, 35, 35, 1);
        }
        else
        {
          tsubpart1 =  new MarcRadioPartClass(0, true, "When option is not active you'll just get a popup window while AI is making moves.",  self.OwnBitmap, num3, num22);
          self.b30id = self.AddSubPart( tsubpart1, num3, num22, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "SHOW AI MOVES", self.game.MarcFont4, num3 + 50, num22 + 8, Color.White);
        let mut num23: i32 = num22 + num5;
        if (!self.game.EditObj.showAdvice)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active the Advise window will be available.",  self.OwnBitmap, num3, num23);
          self.b32id = self.AddSubPart( tsubpart1, num3, num23, 35, 35, 1);
        }
        else
        {
          tsubpart1 =  new MarcRadioPartClass(0, true, "When option is not active no Advise window will be available.",  self.OwnBitmap, num3, num23);
          self.b32id = self.AddSubPart( tsubpart1, num3, num23, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "SHOW ADVICE", self.game.MarcFont4, num3 + 50, num23 + 8, Color.White);
        num4 = num23 + num5;
      }
      if (self.currentCat != 4)
        return;
      let mut num24: i32 = 1;
      let mut num25: i32 = 1024;
      if (!self.game.EditObj.DoubleSize)
      {
        if (self.game.Data.Product == 7)
        {
          num25 = 1280;
          if (!( self.game.RealScreenWidth >= 1600.0 *  num24 &  self.game.RealScreenHeight >= 960.0 *  num24))
            DrawMod.DrawTextColouredMarc( g, "Your need to have a resolution equal or above 1280x960 to have >100% DPI prefs. ", self.game.MarcFont4, num3, num4, Color.White);
        }
        else if (!( self.game.RealScreenWidth >= 1280.0 *  num24 &  self.game.RealScreenHeight >= 960.0 *  num24))
          DrawMod.DrawTextColouredMarc( g, "Your need to have a resolution equal or above 1280x960 to have >100% DPI prefs. ", self.game.MarcFont4, num3, num4, Color.White);
      }
      if (self.game.EditObj.DoubleSizePercentage != 75)
      {
        if (!self.game.EditObj.DoubleSize &  self.game.RealScreenWidth >=  num25 * 1.25 *  num24 &  self.game.RealScreenHeight >= 960.0 *  num24)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active your screen resolution will appear to be 7/8 of your real resolution.",  self.OwnBitmap, num3, num4);
          self.B17bId = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "7/8 RESOLUTION (125% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        else if (self.game.EditObj.DoubleSize & self.game.EditObj.DoubleSizePercentage == 125)
        {
          tsubpart1 =  new MarcRadioPartClass(0, true, "When option is active your screen resolution will appear to be 7/8 of your real resolution. ",  self.OwnBitmap, num3, num4);
          self.B17bId = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "7/8 RESOLUTION (125% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        else if (self.game.EditObj.DoubleSize & self.game.EditObj.DoubleSizePercentage != 125 &  self.game.RealScreenWidth >=  num25 * 1.25 *  num24 &  self.game.RealScreenHeight >= 960.0 *  num24)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active your screen resolution will appear to be 7/8 of your real resolution. ",  self.OwnBitmap, num3, num4);
          self.B17bId = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "7/8 RESOLUTION (125% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        num4 += num5;
        if (!self.game.EditObj.DoubleSize &  self.game.RealScreenWidth >=  num25 * 1.5 *  num24 &  self.game.RealScreenHeight >= 1152.0 *  num24)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active your screen resolution will appear to be 3/4 of your real resolution.",  self.OwnBitmap, num3, num4);
          self.B17cId = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "3/4 RESOLUTION (150% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        else if (self.game.EditObj.DoubleSize & self.game.EditObj.DoubleSizePercentage == 150)
        {
          tsubpart1 =  new MarcRadioPartClass(0, true, "When option is active your screen resolution will appear to be 3/4 of your real resolution. ",  self.OwnBitmap, num3, num4);
          self.B17cId = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "3/4 RESOLUTION (150% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        else if (self.game.EditObj.DoubleSize & self.game.EditObj.DoubleSizePercentage != 150 &  self.game.RealScreenWidth >=  num25 * 1.5 *  num24 &  self.game.RealScreenHeight >= 1152.0 *  num24)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active your screen resolution will appear to be 3/4 of your real resolution. ",  self.OwnBitmap, num3, num4);
          self.B17cId = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "3/4 RESOLUTION (150% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        if (self.B17cId > 0)
          num4 += num5;
        if (!self.game.EditObj.DoubleSize &  self.game.RealScreenWidth >=  num25 * 1.75 *  num24 &  self.game.RealScreenHeight >= 1344.0 *  num24)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active your screen resolution will appear to be 5/8 of your real resolution.",  self.OwnBitmap, num3, num4);
          self.B17dId = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "5/8 RESOLUTION (175% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        else if (self.game.EditObj.DoubleSize & self.game.EditObj.DoubleSizePercentage == 175)
        {
          tsubpart1 =  new MarcRadioPartClass(0, true, "When option is active your screen resolution will appear to be 5/8 of your real resolution. ",  self.OwnBitmap, num3, num4);
          self.B17dId = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "5/8 RESOLUTION (175% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        else if (self.game.EditObj.DoubleSize & self.game.EditObj.DoubleSizePercentage != 175 &  self.game.RealScreenWidth >=  num25 * 1.75 *  num24 &  self.game.RealScreenHeight >= 1344.0 *  num24)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active your screen resolution will appear to be 5/8 of your real resolution. ",  self.OwnBitmap, num3, num4);
          self.B17dId = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "5/8 RESOLUTION (175% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        if (self.B17dId > 0)
          num4 += num5;
        if (!self.game.EditObj.DoubleSize & self.game.RealScreenWidth >= num25 * 2 * num24 & self.game.RealScreenHeight >= 1536 * num24)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active your screen resolution will appear to be 1/2 of your real resolution.",  self.OwnBitmap, num3, num4);
          self.B17Id = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "1/2 RESOLUTION (200% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        else if (self.game.EditObj.DoubleSize & self.game.EditObj.DoubleSizePercentage == 200)
        {
          tsubpart1 =  new MarcRadioPartClass(0, true, "When option is active your screen resolution will appear to be 1/2 of your real resolution. ",  self.OwnBitmap, num3, num4);
          self.B17Id = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "1/2 RESOLUTION (200% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        else if (self.game.EditObj.DoubleSize & self.game.EditObj.DoubleSizePercentage != 200 & self.game.RealScreenWidth >= num25 * 2 * num24 & self.game.RealScreenHeight >= 1536 * num24)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active your screen resolution will appear to be 1/2 of your real resolution. ",  self.OwnBitmap, num3, num4);
          self.B17Id = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
          DrawMod.DrawTextColouredMarc( g, "1/2 RESOLUTION (200% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
        }
        else
        {
          let mut num26: i32 = self.game.SuperAdminRights ? 1 : 0;
        }
        if (self.B17Id > 0)
          num4 += num5;
      }
      if (self.game.EditObj.DoubleSize & self.game.EditObj.DoubleSizePercentage == 75)
      {
        tsubpart1 =  new MarcRadioPartClass(0, true, "When option is active your screen resolution will appear to be larger than your real resolution. Will result in some loss of quality and readability.",  self.OwnBitmap, num3, num4);
        self.B17eId = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
      }
      else
      {
        tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active your screen resolution will appear to be larger than your real resolution. Will result in some loss of quality and readability.",  self.OwnBitmap, num3, num4);
        self.B17eId = self.AddSubPart( tsubpart1, num3, num4, 35, 35, 1);
      }
      DrawMod.DrawTextColouredMarc( g, "5/4 RESOLUTION (75% DPI)", self.game.MarcFont4, num3 + 50, num4 + 8, Color.White);
      let mut num27: i32 = num4 + num5;
      if (self.game.EventRelatedObj.Helper_IsDebug())
      {
        if (self.game.EditObj.DoubleSize & self.game.EditObj.DoubleSizePercentage == 50)
        {
          tsubpart1 =  new MarcRadioPartClass(0, true, "When option is active your screen resolution will appear to be larger than your real resolution. Will result in some loss of quality and readability.",  self.OwnBitmap, num3, num27);
          self.B17fId = self.AddSubPart( tsubpart1, num3, num27, 35, 35, 1);
        }
        else
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "When option is active your screen resolution will appear to be larger than your real resolution. Will result in some loss of quality and readability.",  self.OwnBitmap, num3, num27);
          self.B17fId = self.AddSubPart( tsubpart1, num3, num27, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "2/1 RESOLUTION (50% DPI)", self.game.MarcFont4, num3 + 50, num27 + 8, Color.White);
        num11 = num27 + num5;
      }
      num28: i32;
      if (!( self.game.RealScreenWidth >=  num25 * 1.25 *  num24 &  self.game.RealScreenHeight >= 960.0 *  num24))
      {
        num28 = num7 + num5 + num5;
      }
      else
      {
        num28 = num7;
        num3 += num6 + 50;
      }
      if (self.game.HighGraphicsSpeedPossible)
      {
        if (!self.game.EditObj.highGfxSpeedOn)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "HIGHSPEED GFX: Presses the most speed out of Windows GDI(+) graphics possible. Advised to leave flagged unless you are experiencing problems. ",  self.OwnBitmap, num3, num28);
          self.B20Id = self.AddSubPart( tsubpart1, num3, num28, 35, 35, 1);
        }
        else
        {
          tsubpart1 =  new MarcRadioPartClass(0, true, "HIGHSPEED GFX: Presses the most speed out of Windows GDI(+) graphics possible. Advised to leave flagged unless you are experiencing problems. ",  self.OwnBitmap, num3, num28);
          self.B20Id = self.AddSubPart( tsubpart1, num3, num28, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "Speed", self.game.MarcFont4, num3 + 35, num28 + 8, Color.White);
        if (!self.game.EditObj.skipGfxDetail)
        {
          tsubpart1 =  new MarcRadioPartClass(0, false, "SKIP GFX DETAILS: Flag to skip some aesthetical details to render the map faster. Only flag if things are to slow. ",  self.OwnBitmap, num3 + 85, num28);
          self.b36id = self.AddSubPart( tsubpart1, num3 + 85, num28, 35, 35, 1);
        }
        else
        {
          tsubpart1 =  new MarcRadioPartClass(0, true, "SKIP GFX DETAILS: Flag to skip some aesthetical details to render the map faster. Only flag if things are to slow. ",  self.OwnBitmap, num3 + 85, num28);
          self.b36id = self.AddSubPart( tsubpart1, num3 + 85, num28, 35, 35, 1);
        }
        DrawMod.DrawTextColouredMarc( g, "Skip", self.game.MarcFont4, num3 + 85 + 35, num28 + 8, Color.White);
        if (!( self.game.RealScreenWidth >=  num25 * 1.25 *  num24 &  self.game.RealScreenHeight >= 960.0 *  num24))
        {
          let mut num29: i32 = num28 + num5;
          if (!self.game.EditObj.mouseScreenLock)
          {
            tsubpart1 =  new MarcRadioPartClass(0, false, "Mouse Lock locks your mouse to the Windows bounds. You might want to disable in certain multi-monitor or bordered-window play modes. ",  self.OwnBitmap, num3, num29);
            self.b31id = self.AddSubPart( tsubpart1, num3, num29, 35, 35, 1);
          }
          else
          {
            tsubpart1 =  new MarcRadioPartClass(0, true, "Mouse Lock locks your mouse to the Windows bounds. You might want to disable in certain multi-monitor or bordered-window play modes. ",  self.OwnBitmap, num3, num29);
            self.b31id = self.AddSubPart( tsubpart1, num3, num29, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "Mouse Lock", self.game.MarcFont4, num3 + 50, num29 + 8, Color.White);
          num28 = num29 + num5;
        }
        else
        {
          if (!self.game.EditObj.mouseScreenLock)
          {
            tsubpart1 =  new MarcRadioPartClass(0, false, "Mouse Lock locks your mouse to the Windows bounds. You might want to disable in certain multi-monitor or bordered-window play modes. ",  self.OwnBitmap, num3 + 160, num28);
            self.b31id = self.AddSubPart( tsubpart1, num3 + 160, num28, 35, 35, 1);
          }
          else
          {
            tsubpart1 =  new MarcRadioPartClass(0, true, "Mouse Lock locks your mouse to the Windows bounds. You might want to disable in certain multi-monitor or bordered-window play modes. ",  self.OwnBitmap, num3 + 160, num28);
            self.b31id = self.AddSubPart( tsubpart1, num3 + 160, num28, 35, 35, 1);
          }
          DrawMod.DrawTextColouredMarc( g, "Mouse Lock", self.game.MarcFont4, num3 + 160 + 40, num28 + 8, Color.White);
          num28 += num5;
        }
      }
      if (!(self.game.Data.Product >= 6 & !self.formref.windowsTxtFormPlease))
        return;
      if (num3 < num6 + 50)
      {
        let mut num30: i32 = num7;
        num3 += num6 + 50;
        num28 = num30 + num5;
      }
      self.OptionsListObj = ListClass::new();
      DEVMODE1 lpDevMode1 = DEVMODE1::new();
      DEVMODE1 lpDevMode2 = DEVMODE1::new();
      self.resList = SimpleList::new();
      Form1.EnumDisplaySettings(0, -1,  lpDevMode2);
      lpDevMode1.dmSize = (short) Marshal.SizeOf( lpDevMode1);
      let mut iModeNum: i32 = 0;
      let mut num31: i32 = -1;
      let mut tlistselect: i32 = -1;
      SimpleList simpleList = SimpleList::new();
      bool flag = false;
      let mut num32: i32 = num31 + 1;
      tname1: String = "Desktop (" + self.game.StartupScreenWidth.ToString() + "x" + self.game.StartupScreenHeight.ToString() + ")";
      if (self.detailnr == -1)
      {
        if (self.game.EditObj.overruleScreenResWidth < 1 && self.game.EditObj.overruleScreenResHeight < 1)
          self.detailnr = num32;
        if (self.game.StartupScreenWidth == lpDevMode2.dmPelsWidth & self.game.StartupScreenHeight == lpDevMode2.dmPelsHeight)
          self.detailnr = num32;
      }
      if (self.game.StartupScreenWidth == lpDevMode2.dmPelsWidth & self.game.StartupScreenHeight == lpDevMode2.dmPelsHeight)
      {
        tname1 += " (Current Res)";
        if (tlistselect == num32)
          flag = true;
      }
      simpleList.Add(self.game.StartupScreenWidth, 1, self.game.StartupScreenHeight, CheckExistence: false);
      self.OptionsListObj.add(tname1, num32);
      if (num32 == self.detailnr)
        tlistselect = num32;
      for (; Form1.EnumDisplaySettings(0, iModeNum,  lpDevMode1); iModeNum += 1)
      {
        if ( lpDevMode2.dmBitsPerPel ==  lpDevMode1.dmBitsPerPel && lpDevMode1.dmPelsWidth >= 1280 & lpDevMode1.dmPelsHeight > 768 && simpleList.FindNr(lpDevMode1.dmPelsWidth, lpDevMode1.dmPelsHeight) == -1)
        {
          num32 += 1;
          if (self.detailnr == -1 && self.game.EditObj.overruleScreenResWidth == lpDevMode1.dmPelsWidth && self.game.EditObj.overruleScreenResHeight == lpDevMode1.dmPelsHeight)
            self.detailnr = num32;
          if (num32 == self.detailnr)
            tlistselect = num32;
          simpleList.Add(lpDevMode1.dmPelsWidth, 1, lpDevMode1.dmPelsHeight, CheckExistence: false);
          tname2: String = lpDevMode1.dmPelsWidth.ToString() + "x" + lpDevMode1.dmPelsHeight.ToString();
          if (lpDevMode1.dmPelsWidth == lpDevMode2.dmPelsWidth & lpDevMode1.dmPelsHeight == lpDevMode2.dmPelsHeight)
          {
            tname2 += " (Current Res)";
            if (tlistselect == num32)
              flag = true;
          }
          self.resList.Add(num32, 1, lpDevMode1.dmPelsWidth, lpDevMode1.dmPelsHeight, CheckExistence: false);
          self.OptionsListObj.add(tname2, num32);
        }
      }
      if (self.optionsListId > 0)
      {
        self.SubPartList[self.SubpartNr(self.optionsListId)].Refresh(self.OptionsListObj, tlistselect);
        self.SubPartFlag[self.SubpartNr(self.optionsListId)] = true;
      }
      else
      {
        tsubpart1 =  new ListSubPartClass(self.OptionsListObj, 5, 250, tlistselect, self.game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: num3, bby: num28, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
        self.optionsListId = self.AddSubPart( tsubpart1, num3, num28, 250, 144, 0);
      }
      let mut num33: i32 = num28 + 144 + 8;
      if (flag)
      {
        tsubpart1 =  new TextButtonPartClass("CHANGE RESOLUTION", 240, "Resolution selected is the current resolution. So you cannot really change to it.",  self.OwnBitmap, num3, num33, true, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.b29id = self.AddSubPart( tsubpart1, num3, num33, 240, 35, 0);
      }
      else
      {
        tsubpart1 =  new TextButtonPartClass("CHANGE RESOLUTION", 240, "Select a desired screen resolution in the listbox and confirm by pressing this button. Resolution will be changed. DPI will reset to default.",  self.OwnBitmap, num3, num33, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.b29id = self.AddSubPart( tsubpart1, num3, num33, 240, 35, 1);
      }
      DrawMod.DrawTextColouredMarc( g, "Switch Real Resolution", self.game.MarcFont4, num3 + 50, num33 + 8, Color.White);
      num11 = num33 + num5;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index] && Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
          {
            self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "";
            self.game.EditObj.TipText = self.SubPartList[index].Descript;
            return;
          }
        }
      }
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          break;
        }
      }
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

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      for (let mut mouseCounter: i32 = self.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (self.MouseData[mouseCounter] > 0 && x > self.MouseRect[mouseCounter].X & x < self.MouseRect[mouseCounter].X + self.MouseRect[mouseCounter].Width && y > self.MouseRect[mouseCounter].Y & y < self.MouseRect[mouseCounter].Y + self.MouseRect[mouseCounter].Height)
        {
          if (self.MouseData[mouseCounter] > 1000)
          {
            self.currentCat = self.MouseData[mouseCounter] - 1000;
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (self.MouseData[mouseCounter] == 999)
          {
            self.game.EditObj.SetViewMode2 = 0;
            windowReturnClass.AddCommand(1, 9);
            windowReturnClass.AddCommand(7, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      let mut mouseCounter1: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter1; index += 1)
      {
        if (self.MouseData[index] > 0 && x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 1000)
          {
            self.currentCat = self.MouseData[index] - 1000;
            self.dostuff();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
          if (self.MouseData[index] == 999)
          {
            self.game.EditObj.SetViewMode2 = 0;
            windowReturnClass.AddCommand(1, 9);
            windowReturnClass.AddCommand(7, 12);
            windowReturnClass.AddCommand(4, 67);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      if (x > 8 & x < self.w - 8 && y < self.h - 24)
        windowReturnClass.NoMouseClickBelow = true;
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num1: i32 = self.SubPartID[index];
            if (num1 == self.optionsListId)
            {
              let mut num2: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              if (num2 <= -1)
                return windowReturnClass;
              self.detailnr = num2;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.slider1id)
            {
              self.game.EditObj.Volume = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              self.SubPartList[index].Scroller = true;
              SoundMod.ChangeEventSoundBg(self.game.EditObj);
              return windowReturnClass;
            }
            if (num1 == self.slider2id)
            {
              self.game.EditObj.Volume2 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              self.SubPartList[index].Scroller = true;
              SoundMod.ChangeEventSound(self.game.EditObj);
              return windowReturnClass;
            }
            if (num1 == self.B1Id)
            {
              self.game.EditObj.SoundOn = !self.game.EditObj.SoundOn;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B19Id)
            {
              self.game.EditObj.AlternateMusic = !self.game.EditObj.AlternateMusic;
              SoundMod.STopEventBackground();
              SoundMod.RestartLastBackground( self.game.EditObj);
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B2Id)
            {
              self.game.EditObj.PrefShowFOW = !self.game.EditObj.PrefShowFOW;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B8Id)
            {
              self.game.EditObj.IntroSoundOn = !self.game.EditObj.IntroSoundOn;
              if (!self.game.EditObj.IntroSoundOn)
              {
                if (self.game.Data.Product == 7)
                {
                  SoundMod.STopEventBackground();
                  SoundMod.dssEnd( self.game.EditObj);
                }
                else
                  SoundMod.STopEventBackground();
              }
              else if (self.game.Data.Product == 7)
              {
                SoundMod.RestartLastBackground( self.game.EditObj);
                SoundMod.dssTimer( self.game.EditObj);
              }
              else
                SoundMod.RestartLastBackground( self.game.EditObj);
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B9Id)
            {
              self.game.EditObj.AutoSave = !self.game.EditObj.AutoSave;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B24Id)
            {
              self.game.EditObj.ShowLISRange = !self.game.EditObj.ShowLISRange;
              if (self.game.EditObj.ShowLISRange)
              {
                self.game.EditObj.ShowHQPower = false;
                self.game.EditObj.ShowAirRange = false;
              }
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B5Id)
            {
              self.game.EditObj.CombatNumbers = !self.game.EditObj.CombatNumbers;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B21Id)
            {
              if (self.game.EditObj.HideUnit == 2)
                self.game.EditObj.HideUnit = 1;
              else
                self.game.EditObj.HideUnit = 2;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.AddCommand(4, 2);
              windowReturnClass.AddCommand(4, 69);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B22Id)
            {
              self.game.EditObj.SpreadUnit = !self.game.EditObj.SpreadUnit;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.AddCommand(4, 2);
              windowReturnClass.AddCommand(4, 69);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B3Id)
            {
              self.game.EditObj.ShowUnderHQ = !self.game.EditObj.ShowUnderHQ;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B6Id)
            {
              self.game.EditObj.ShowSameHistorical = !self.game.EditObj.ShowSameHistorical;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B7Id)
            {
              self.game.EditObj.ShowMouseOver = !self.game.EditObj.ShowMouseOver;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b26id)
            {
              self.game.EditObj.AutoCombat = !self.game.EditObj.AutoCombat;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b27id)
            {
              self.game.EditObj.maximumInterfaceSpace = !self.game.EditObj.maximumInterfaceSpace;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b31id)
            {
              self.game.EditObj.mouseScreenLock = !self.game.EditObj.mouseScreenLock;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b28id)
            {
              self.game.EditObj.useLeftRightClickMode = !self.game.EditObj.useLeftRightClickMode;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b30id)
            {
              self.game.EditObj.dontShowAImoves = !self.game.EditObj.dontShowAImoves;
              self.game.Data.DontShowAIMove = self.game.EditObj.dontShowAImoves;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b32id)
            {
              self.game.EditObj.showAdvice = !self.game.EditObj.showAdvice;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b33id)
            {
              let mut stringListById: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 156, 0, 0));
              let mut setValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, 56, 2))) == 1 ? 0 : 1;
              self.game.Data.StringListObj[stringListById].SetData(0, 56, 2, setValue);
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b34id)
            {
              let mut stringListById: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 156, 0, 0));
              let mut setValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, 56, 2))) == 2 ? 0 : 2;
              self.game.Data.StringListObj[stringListById].SetData(0, 56, 2, setValue);
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b35id)
            {
              let mut stringListById: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 156, 0, 0));
              let mut setValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById].GetData(0, 56, 2))) == 0 ? 0 : 0;
              self.game.Data.StringListObj[stringListById].SetData(0, 56, 2, setValue);
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B10Id)
            {
              self.game.EditObj.ShowLabel = !self.game.EditObj.ShowLabel;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B11Id)
            {
              self.game.EditObj.HideAS = !self.game.EditObj.HideAS;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b36id)
            {
              self.game.EditObj.skipGfxDetail = !self.game.EditObj.skipGfxDetail;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B12Id)
            {
              self.game.EditObj.ShowHQPower = !self.game.EditObj.ShowHQPower;
              if (self.game.EditObj.ShowHQPower)
                self.game.EditObj.ShowLISRange = false;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B16Id)
            {
              self.game.EditObj.RegimeColoring = !self.game.EditObj.RegimeColoring;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B13Id)
            {
              self.game.EditObj.Screenshoton = !self.game.EditObj.Screenshoton;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B20Id)
            {
              self.game.EditObj.highGfxSpeedOn = !self.game.EditObj.highGfxSpeedOn;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B4Id)
            {
              self.game.EditObj.HexRasterOn = !self.game.EditObj.HexRasterOn;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B14Id)
            {
              self.game.EditObj.ShowAirRange = !self.game.EditObj.ShowAirRange;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B15Id)
            {
              self.game.EditObj.PrefMinimalistCounter = !self.game.EditObj.PrefMinimalistCounter;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B18Id)
            {
              self.game.EditObj.allowMetrics = !self.game.EditObj.allowMetrics;
              self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b29id)
            {
              if (self.detailnr == 0)
              {
                self.game.HandyFunctionsObj.SwitchResolution2( self.formref, self.game.StartupScreenWidth, self.game.StartupScreenHeight, true);
                windowReturnClass.AddCommand(3, 11);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (!Information.IsNothing( self.resList) & self.detailnr > -1)
              {
                let mut nr: i32 = self.resList.FindNr(self.detailnr);
                if (nr > -1)
                {
                  self.game.HandyFunctionsObj.SwitchResolution2( self.formref, self.resList.Data1[nr], self.resList.Data2[nr], false);
                  windowReturnClass.AddCommand(3, 11);
                  windowReturnClass.SetFlag(true);
                }
              }
              return windowReturnClass;
            }
            if (num1 == self.B17bId)
            {
              if (self.game.EditObj.DoubleSizePercentage == 125)
              {
                self.game.EditObj.DoubleSize = false;
                self.game.EditObj.DoubleSizePercentage = 100;
              }
              else
              {
                self.game.EditObj.DoubleSize = true;
                self.game.EditObj.DoubleSizePercentage = 125;
              }
              self.game.HandyFunctionsObj.SwitchResolution( self.formref);
              windowReturnClass.AddCommand(3, 11);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B17cId)
            {
              if (self.game.EditObj.DoubleSizePercentage == 150)
              {
                self.game.EditObj.DoubleSize = false;
                self.game.EditObj.DoubleSizePercentage = 100;
              }
              else
              {
                self.game.EditObj.DoubleSize = true;
                self.game.EditObj.DoubleSizePercentage = 150;
              }
              self.game.HandyFunctionsObj.SwitchResolution( self.formref);
              windowReturnClass.AddCommand(3, 11);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B17dId)
            {
              if (self.game.EditObj.DoubleSizePercentage == 175)
              {
                self.game.EditObj.DoubleSize = false;
                self.game.EditObj.DoubleSizePercentage = 100;
              }
              else
              {
                self.game.EditObj.DoubleSize = true;
                self.game.EditObj.DoubleSizePercentage = 175;
              }
              self.game.HandyFunctionsObj.SwitchResolution( self.formref);
              windowReturnClass.AddCommand(3, 11);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B17eId)
            {
              if (self.game.EditObj.DoubleSizePercentage == 75)
              {
                self.game.EditObj.DoubleSize = false;
                self.game.EditObj.DoubleSizePercentage = 100;
              }
              else
              {
                self.game.EditObj.DoubleSize = true;
                self.game.EditObj.DoubleSizePercentage = 75;
              }
              self.game.HandyFunctionsObj.SwitchResolution( self.formref);
              windowReturnClass.AddCommand(3, 11);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B17fId)
            {
              if (self.game.EditObj.DoubleSizePercentage == 50)
              {
                self.game.EditObj.DoubleSize = false;
                self.game.EditObj.DoubleSizePercentage = 100;
              }
              else
              {
                self.game.EditObj.DoubleSize = true;
                self.game.EditObj.DoubleSizePercentage = 50;
              }
              self.game.HandyFunctionsObj.SwitchResolution( self.formref);
              windowReturnClass.AddCommand(3, 11);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B17Id)
            {
              if (self.game.EditObj.DoubleSizePercentage == 200)
              {
                self.game.EditObj.DoubleSize = false;
                self.game.EditObj.DoubleSizePercentage = 100;
              }
              else
              {
                self.game.EditObj.DoubleSize = true;
                self.game.EditObj.DoubleSizePercentage = 200;
              }
              self.game.HandyFunctionsObj.SwitchResolution( self.formref);
              windowReturnClass.AddCommand(3, 11);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b25id)
            {
              if (Interaction.MsgBox( "Quitting... are you sure?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              {
                self.game.Data = DataClass::new();
                self.game.EditObj = new EditClass(self.game.AppPath + "editobj.txt");
                self.game.EditObj.ShowInitialMenu = true;
                windowReturnClass.AddCommand(3, 12);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == self.minId)
              {
                self.game.FormRef.WindowState = FormWindowState.Minimized;
                return windowReturnClass;
              }
              if (num1 == self.B23Id)
              {
                str1: String;
                if (self.game.Data.Round == 0)
                {
                  str2: String = self.game.AppPath + "scenarios\\";
                  if (!Information.IsNothing( self.game.Data.ScenarioDir))
                  {
                    if (self.game.Data.ScenarioDir.Length > 1)
                      str1 = str2.Replace("scenarios", self.game.Data.ScenarioDir);
                    else if (self.game.ModScenarioDir.Length > 1)
                      str1 = str2.Replace("scenarios", self.game.ModScenarioDir);
                  }
                  else if (self.game.ModScenarioDir.Length > 1)
                    str1 = str2.Replace("scenarios", self.game.ModScenarioDir);
                }
                else
                  str1 = "savedgames";
                str3: String = self.game.Data.Round != 0 ? self.game.HandyFunctionsObj.SaveSomething("SE1 Scenario file (*.se1)|*.se1", "Give save name...", self.game.AppPath_SAVEGAMES, false) : self.game.HandyFunctionsObj.SaveSomething("SE1 Scenario file (*.se1)|*.se1", "Give save name...", self.game.AppPath_SAVEGAMES, false);
                if (Strings.Len(str3) >= 2)
                {
                  self.game.FormRef.Cursor = Cursors.WaitCursor;
                  self.game.Data.serialize(str3);
                  self.game.HandyFunctionsObj.ZipFile(str3);
                  windowReturnClass.SetFlag(true);
                  self.game.FormRef.Cursor = Cursors.Default;
                }
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
