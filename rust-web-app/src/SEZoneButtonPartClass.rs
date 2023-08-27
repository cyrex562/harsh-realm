// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SEZoneButtonPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SEZoneButtonPartClass : SubPartClass
  {
     iconSlotNr: i32;
     dataString: String;
     description: String;
     bool active;
     delta: i32;
     bool showDelta;
     eventpicslotPos: i32;
     eventpicslotNeg: i32;
     eventpicslotZero: i32;

    pub fn SubDispose()
    {
    }

    pub SEZoneButtonPartClass(
      tIconSlotNr: i32,
      tDataString: String,
      tDescript: String,
      bool tactive,
      bool tshowDelta = false,
      let mut tdelta: i32 = 0)
      : base(220, 40)
    {
      self.iconSlotNr = tIconSlotNr;
      self.Descript = tDescript;
      self.dataString = tDataString;
      self.active = tactive;
      self.showDelta = tshowDelta;
      self.delta = tdelta;
      self.eventpicslotPos = DrawMod.TGame.Data.FindEventPic("", 8, "SE_Present");
      self.eventpicslotNeg = DrawMod.TGame.Data.FindEventPic("", 9, "SE_Present");
      self.eventpicslotZero = DrawMod.TGame.Data.FindEventPic("", 11, "SE_Present");
    }

    pub Paint: Bitmap()
    {
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      bitmap: Bitmap;
      if (!self.active)
      {
         let mut local1: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ZONEBUTTON);
         let mut local2: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local1,  local2, 0, 0);
      }
      if (self.active)
      {
         let mut local3: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ZONEBUTTONHIGH);
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
        rectangle2 = Rectangle::new(0, 3, 48, 32);
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
        rectangle1 = Rectangle::new(0, 3, 48, 32);
        let mut destrect: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local7,  local8, srcrect, destrect);
      }
      if (!self.active)
        DrawMod.DrawTextColouredConsole( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 51, 11, DrawMod.TGame.seColGray);
      if (self.active)
        DrawMod.DrawTextColouredConsole( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 51, 11, DrawMod.TGame.seColWhite);
      if (self.showDelta)
      {
        bool flag = false;
        SizeF sizeF = SizeF::new();
        let mut num1: i32 = self.delta;
        if (num1 < 0)
        {
          num1 = Math.Abs(num1);
          flag = true;
        }
        str: String = num1.ToString();
        let mut num2: i32 =  Math.Round(210.0 - ( objgraphics.MeasureString(str, DrawMod.TGame.MarcFont16).Width + 4.0 + 16.0));
        let mut index: i32 = self.eventpicslotZero;
        if (flag)
          index = self.eventpicslotNeg;
        else if (self.delta > 0)
          index = self.eventpicslotPos;
         let mut local9: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.Data.EventPicNr[index]);
         let mut local10: &Bitmap = &bitmap;
        let mut x1: i32 = num2;
        DrawMod.DrawSimple( local9,  local10, x1, 13);
        let mut x2: i32 = num2 + 15;
        DrawMod.DrawTextColouredConsole( objgraphics, str, DrawMod.TGame.MarcFont16, x2, 11, DrawMod.TGame.seColGray);
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
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ZONEBUTTON);
         let mut local2: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local1,  local2, 0, 0);
         let mut local3: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ZONEBUTTONHIGH);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.Draw( local3,  local4, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
      }
      if (self.active)
      {
         let mut local5: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ZONEBUTTONHIGH);
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
        rectangle2 = Rectangle::new(0, 3, 48, 32);
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
        rectangle1 = Rectangle::new(0, 3, 48, 32);
        let mut destrect: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local9,  local10, srcrect, destrect);
      }
      if (!self.active)
        DrawMod.DrawTextColouredConsole( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 51, 11, DrawMod.TGame.seColGray);
      if (self.active)
        DrawMod.DrawTextColouredConsole( objgraphics, self.dataString, DrawMod.TGame.MarcFont16, 51, 11, DrawMod.TGame.seColWhite);
      if (self.showDelta)
      {
        bool flag = false;
        SizeF sizeF = SizeF::new();
        let mut num1: i32 = self.delta;
        if (num1 < 0)
        {
          num1 = Math.Abs(num1);
          flag = true;
        }
        str: String = num1.ToString();
        let mut num2: i32 =  Math.Round(210.0 - ( objgraphics.MeasureString(str, DrawMod.TGame.MarcFont16).Width + 4.0 + 16.0));
        let mut index: i32 = self.eventpicslotZero;
        if (flag)
          index = self.eventpicslotNeg;
        else if (self.delta > 0)
          index = self.eventpicslotPos;
         let mut local11: &Graphics = &objgraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.Data.EventPicNr[index]);
         let mut local12: &Bitmap = &bitmap;
        let mut x1: i32 = num2;
        DrawMod.DrawSimple( local11,  local12, x1, 13);
        let mut x2: i32 = num2 + 15;
        DrawMod.DrawTextColouredConsole( objgraphics, str, DrawMod.TGame.MarcFont16, x2, 11, DrawMod.TGame.seColGray);
      }
      if (!Information.IsNothing( objgraphics))
      {
        objgraphics.Dispose();
        objgraphics = (Graphics) null;
      }
      return self.OwnBitmap;
    }
  }
}
