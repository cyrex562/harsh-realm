// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SystemMessageWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SystemMessageWindowClass : WindowClass
  {
     okid: i32;
     cancelid: i32;
     oktextid: i32;
     Pic1Id: i32;
     TAid: i32;
     His: i32;
     Card: i32;
     Unr: i32;

    pub SystemMessageWindowClass( tGame: GameClass)
      : base( tGame, 600, 200, 8)
    {
      self.View();
    }

    pub fn View()
    {
      self.ClearMouse();
      self.NewBackGroundAndClearAll(600, 200, -1);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawMessFrame( self.OwnBitmap,  graphics, 0, 0, 600, 200);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      SizeF sizeF1 = SizeF::new();
      str1: String = self.game.EditObj.FeedBackString;
      sizeF1 = graphics.MeasureString(str1, self.game.MarcFont4);
      let mut num1: i32 = -1;
      while (Strings.Len(str1) > 0)
      {
        num1 += 1;
        let mut Length: i32 = Strings.InStr(str1, ".");
        let mut num2: i32 = 0;
        if (Length > 0)
          num2 = Strings.InStr(Length + 1, str1, ".");
        str2: String;
        if (Length > 0 & num2 > 0)
        {
          str2 = Strings.Left(str1, Length);
          str1 = Strings.Mid(str1, Length + 1);
        }
        else
        {
          str2 = str1;
          str1 = "";
        }
        SizeF sizeF2 = graphics.MeasureString(str2, self.game.MarcFont4);
        DrawMod.DrawTextColouredMarc( graphics, str2, self.game.MarcFont4,  Math.Round(300.0 -  sizeF2.Width / 2.0), 40 + num1 * 20, Color.White);
      }
      let mut tsubpart: SubPartClass =  new TextButtonPartClass("OK", 200, tBackbitmap: ( self.OwnBitmap), bbx: 200, bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.cancelid = self.AddSubPart( tsubpart, 200, 100, 200, 36, 1);
      Rectangle trect = Rectangle::new(200, 100, 200, 35);
      self.AddMouse( trect, "", "Close this message");
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          windowReturnClass.AddCommand(6, 0);
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
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index] && self.SubPartID[index] == self.cancelid)
          {
            windowReturnClass.AddCommand(6, 0);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
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
