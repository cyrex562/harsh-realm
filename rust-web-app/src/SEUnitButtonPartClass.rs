// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SEUnitButtonPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SEUnitButtonPartClass : SubPartClass
  {
     iconSlotNr: i32;
     dataString: String;
     dataString2: String;
     description: String;
     bool active;
     overruleR: i32;
     overruleG: i32;
     overruleB: i32;

    pub fn SubDispose()
    {
    }

    pub SEUnitButtonPartClass(
      tIconSlotNr: i32,
      tDataString: String,
      tDataString2: String,
      tDescript: String,
      bool tactive,
      let mut tOverruleR: i32 = -1,
      let mut tOverruleG: i32 = -1,
      let mut tOverruleB: i32 = -1)
      : base(44, 97)
    {
      self.iconSlotNr = tIconSlotNr;
      self.Descript = tDescript;
      self.dataString = tDataString;
      self.dataString2 = tDataString2;
      self.active = tactive;
      self.overruleG = tOverruleG;
      self.overruleB = tOverruleB;
      self.overruleR = tOverruleR;
    }

    pub Paint: Bitmap()
    {
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      bitmap: Bitmap;
      if (!self.active)
      {
         let mut local1: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_VARBOX5);
         let mut local2: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local1,  local2, 0, 0);
      }
      if (self.active)
      {
         let mut local3: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_VARBOX5HIGH);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local3,  local4, 0, 0);
      }
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (!self.active)
      {
         let mut local5: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
         let mut local6: &Bitmap = &bitmap;
        rectangle1 = Rectangle::new(self.iconSlotNr * 42, 0, 42, 32);
        let mut srcrect: &Rectangle = &rectangle1
        rectangle2 = Rectangle::new(2, 10, 42, 32);
        let mut destrect: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2( local5,  local6, srcrect, destrect);
      }
      if (self.active)
      {
         let mut local7: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
         let mut local8: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(self.iconSlotNr * 42, 32, 42, 32);
        let mut srcrect: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(2, 10, 42, 32);
        let mut destrect: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local7,  local8, srcrect, destrect);
      }
      if (self.overruleB > -1)
      {
        if (self.dataString2.Length > 0)
        {
          if (!self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 51, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
          if (self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 51, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
          if (!self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString2, DrawMod.TGame.MarcFont16, 22, 65, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
          if (self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString2, DrawMod.TGame.MarcFont16, 22, 65, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
        }
        else
        {
          if (!self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 58, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
          if (self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 58, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
        }
      }
      else if (self.dataString2.Length > 0)
      {
        if (!self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 51, DrawMod.TGame.seColGray);
        if (self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 51, DrawMod.TGame.seColWhite);
        if (!self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString2, DrawMod.TGame.MarcFont16, 22, 65, DrawMod.TGame.seColGray);
        if (self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString2, DrawMod.TGame.MarcFont16, 22, 65, DrawMod.TGame.seColWhite);
      }
      else
      {
        if (!self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 58, DrawMod.TGame.seColGray);
        if (self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 58, DrawMod.TGame.seColWhite);
      }
      if (!Information.IsNothing( objgraphics))
        objgraphics.Dispose();
      return self.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      bitmap: Bitmap;
      if (!self.active)
      {
         let mut local1: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_VARBOX5);
         let mut local2: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local1,  local2, 0, 0);
         let mut local3: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_VARBOX5HIGH);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.Draw( local3,  local4, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
      }
      if (self.active)
      {
         let mut local5: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_VARBOX5HIGH);
         let mut local6: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local5,  local6, 0, 0);
      }
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (!self.active)
      {
         let mut local7: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
         let mut local8: &Bitmap = &bitmap;
        rectangle1 = Rectangle::new(self.iconSlotNr * 42, 0, 42, 32);
        let mut srcrect: &Rectangle = &rectangle1
        rectangle2 = Rectangle::new(2, 10, 42, 32);
        let mut destrect: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2( local7,  local8, srcrect, destrect);
      }
      if (self.active)
      {
         let mut local9: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
         let mut local10: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(self.iconSlotNr * 42, 32, 42, 32);
        let mut srcrect: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(2, 10, 42, 32);
        let mut destrect: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local9,  local10, srcrect, destrect);
      }
      if (self.overruleB > -1)
      {
        if (self.dataString2.Length > 0)
        {
          if (!self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 51, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
          if (self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 51, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
          if (!self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString2, DrawMod.TGame.MarcFont16, 22, 65, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
          if (self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString2, DrawMod.TGame.MarcFont16, 22, 65, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
        }
        else
        {
          if (!self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 58, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
          if (self.active)
            DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 58, Color.FromArgb( byte.MaxValue, self.overruleR, self.overruleG, self.overruleB));
        }
      }
      else if (self.dataString2.Length > 0)
      {
        if (!self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 51, DrawMod.TGame.seColGray);
        if (self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 51, DrawMod.TGame.seColWhite);
        if (!self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString2, DrawMod.TGame.MarcFont16, 22, 65, DrawMod.TGame.seColGray);
        if (self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString2, DrawMod.TGame.MarcFont16, 22, 65, DrawMod.TGame.seColWhite);
      }
      else
      {
        if (!self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 58, DrawMod.TGame.seColGray);
        if (self.active)
          DrawMod.DrawTextColouredConsoleCenter( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 22, 58, DrawMod.TGame.seColWhite);
      }
      if (!Information.IsNothing( objgraphics))
        objgraphics.Dispose();
      return self.OwnBitmap;
    }
  }
}
