// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SEZoneButtonShortPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SEZoneButtonShortPartClass : SubPartClass
  {
     iconSlotNr: i32;
     dataString: String;
     bool active;
     customIconBmpNr: i32;

    pub fn SubDispose()
    {
    }

    pub SEZoneButtonShortPartClass(
      tIconSlotNr: i32,
      tCustomIconBitmapNr: i32,
      tDataString: String,
      tDescript: String,
      bool tactive)
      : base(93, 40)
    {
      self.iconSlotNr = tIconSlotNr;
      self.Descript = tDescript;
      self.dataString = tDataString;
      self.active = tactive;
      self.customIconBmpNr = tCustomIconBitmapNr;
    }

    pub Paint: Bitmap()
    {
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      bitmap: Bitmap;
      if (!self.active)
      {
         let mut local1: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ZONEBUTTONSMALL);
         let mut local2: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local1,  local2, 0, 0);
      }
      if (self.active)
      {
         let mut local3: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ZONEBUTTONSMALLHIGH);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local3,  local4, 0, 0);
      }
      if (self.customIconBmpNr > 0)
      {
        Rectangle rectangle1;
        Rectangle rectangle2;
        if (!self.active)
        {
           let mut local5: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(self.customIconBmpNr);
           let mut local6: &Bitmap = &bitmap;
          rectangle1 = Rectangle::new(0, 0, 42, 32);
          let mut srcrect: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(0, 3, 48, 32);
          let mut destrect: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2( local5,  local6, srcrect, destrect);
        }
        if (self.active)
        {
           let mut local7: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(self.customIconBmpNr);
           let mut local8: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 32, 42, 32);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(0, 3, 48, 32);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local7,  local8, srcrect, destrect);
        }
      }
      else
      {
        Rectangle rectangle3;
        Rectangle rectangle4;
        if (!self.active)
        {
           let mut local9: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
           let mut local10: &Bitmap = &bitmap;
          rectangle3 = Rectangle::new(self.iconSlotNr * 42, 0, 42, 32);
          let mut srcrect: &Rectangle = &rectangle3
          rectangle4 = Rectangle::new(0, 3, 48, 32);
          let mut destrect: &Rectangle = &rectangle4
          DrawMod.DrawSimplePart2( local9,  local10, srcrect, destrect);
        }
        if (self.active)
        {
           let mut local11: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
           let mut local12: &Bitmap = &bitmap;
          rectangle3 = Rectangle::new(self.iconSlotNr * 42, 32, 42, 32);
          let mut srcrect: &Rectangle = &rectangle3
          rectangle4 = Rectangle::new(0, 3, 48, 32);
          let mut destrect: &Rectangle = &rectangle4
          DrawMod.DrawSimplePart2( local11,  local12, srcrect, destrect);
        }
      }
      if (!self.active)
        DrawMod.DrawTextColouredConsole( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 51, 11, DrawMod.TGame.seColGray);
      if (self.active)
        DrawMod.DrawTextColouredConsole( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 51, 11, DrawMod.TGame.seColWhite);
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
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ZONEBUTTONSMALL);
         let mut local2: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local1,  local2, 0, 0);
         let mut local3: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ZONEBUTTONSMALLHIGH);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.Draw( local3,  local4, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
      }
      if (self.active)
      {
         let mut local5: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ZONEBUTTONSMALLHIGH);
         let mut local6: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local5,  local6, 0, 0);
      }
      if (self.customIconBmpNr > 0)
      {
        Rectangle rectangle1;
        Rectangle rectangle2;
        if (!self.active)
        {
           let mut local7: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(self.customIconBmpNr);
           let mut local8: &Bitmap = &bitmap;
          rectangle1 = Rectangle::new(0, 0, 42, 32);
          let mut srcrect: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(0, 3, 48, 32);
          let mut destrect: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2( local7,  local8, srcrect, destrect);
        }
        if (self.active)
        {
           let mut local9: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(self.customIconBmpNr);
           let mut local10: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 32, 42, 32);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(0, 3, 48, 32);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local9,  local10, srcrect, destrect);
        }
      }
      else
      {
        Rectangle rectangle3;
        Rectangle rectangle4;
        if (!self.active)
        {
           let mut local11: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
           let mut local12: &Bitmap = &bitmap;
          rectangle3 = Rectangle::new(self.iconSlotNr * 42, 0, 42, 32);
          let mut srcrect: &Rectangle = &rectangle3
          rectangle4 = Rectangle::new(0, 3, 48, 32);
          let mut destrect: &Rectangle = &rectangle4
          DrawMod.DrawSimplePart2( local11,  local12, srcrect, destrect);
        }
        if (self.active)
        {
           let mut local13: &Graphics = &objgraphics;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
           let mut local14: &Bitmap = &bitmap;
          rectangle3 = Rectangle::new(self.iconSlotNr * 42, 32, 42, 32);
          let mut srcrect: &Rectangle = &rectangle3
          rectangle4 = Rectangle::new(0, 3, 48, 32);
          let mut destrect: &Rectangle = &rectangle4
          DrawMod.DrawSimplePart2( local13,  local14, srcrect, destrect);
        }
      }
      if (!self.active)
        DrawMod.DrawTextColouredConsole( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 51, 11, DrawMod.TGame.seColGray);
      if (self.active)
        DrawMod.DrawTextColouredConsole( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 51, 11, DrawMod.TGame.seColWhite);
      if (!Information.IsNothing( objgraphics))
        objgraphics.Dispose();
      return self.OwnBitmap;
    }
  }
}
