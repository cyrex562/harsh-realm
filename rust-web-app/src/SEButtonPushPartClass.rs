// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SEButtonPushPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SEButtonPushPartClass : SubPartClass
  {
     iconNr: i32;
     width: i32;
     height: i32;
     bool gray;
     bool pushed;

    pub fn SubDispose()
    {
    }

    pub SEButtonPushPartClass(
      ticonnr: i32,
      bool tPushed,
      tDescript: String = "",
      let mut twidth: i32 = 35,
      let mut theight: i32 = 35,
      bool tgrayedOut = false)
      : base(twidth, theight)
    {
      self.width = twidth;
      self.height = theight;
      self.Descript = tDescript;
      self.iconNr = ticonnr;
      self.gray = tgrayedOut;
      self.pushed = tPushed;
    }

    pub Paint: Bitmap()
    {
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      bitmap: Bitmap;
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (self.pushed)
      {
         let mut local1: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ORDERBUTTONPRESSED);
         let mut local2: &Bitmap = &bitmap;
        rectangle1 = Rectangle::new(14, 0, 16, 38);
        let mut srcrect1: &Rectangle = &rectangle1
        rectangle2 = Rectangle::new(14, 0, self.width - 28, self.height);
        let mut destrect1: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
         let mut local3: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ORDERBUTTONPRESSED);
         let mut local4: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(0, 0, 14, 38);
        let mut srcrect2: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(0, 0, 14, self.height);
        let mut destrect2: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
         let mut local5: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ORDERBUTTONPRESSED);
         let mut local6: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(30, 0, 14, 38);
        let mut srcrect3: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(self.width - 14, 0, 14, self.height);
        let mut destrect3: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
      }
      else
      {
         let mut local7: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ARROWBUTTON);
         let mut local8: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(14, 0, 7, 35);
        let mut srcrect4: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(14, 0, self.width - 28, self.height);
        let mut destrect4: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local7,  local8, srcrect4, destrect4);
         let mut local9: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ARROWBUTTON);
         let mut local10: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(0, 0, 14, 35);
        let mut srcrect5: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(0, 0, 14, self.height);
        let mut destrect5: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local9,  local10, srcrect5, destrect5);
         let mut local11: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ARROWBUTTON);
         let mut local12: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(21, 0, 14, 35);
        let mut srcrect6: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(self.width - 14, 0, 14, self.height);
        let mut destrect6: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local11,  local12, srcrect6, destrect6);
      }
      let mut x: i32 =  Math.Round( (self.width - 48) / 2.0);
      let mut y: i32 =  Math.Round( (self.height - 32) / 2.0);
      if (self.gray)
      {
         let mut local13: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
         let mut local14: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(self.iconNr * 42, 0, 42, 32);
        let mut srcrect: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x, y, 48, 32);
        let mut destrect: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2ColouredOld( local13,  local14, srcrect, destrect, 0.3f, 0.3f, 0.3f, 1f);
      }
      else
      {
         let mut local15: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
         let mut local16: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(self.iconNr * 42, 0, 42, 32);
        let mut srcrect: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x, y, 48, 32);
        let mut destrect: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local15,  local16, srcrect, destrect);
      }
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return self.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      bitmap: Bitmap;
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (self.pushed)
      {
         let mut local1: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ORDERBUTTONPRESSED);
         let mut local2: &Bitmap = &bitmap;
        rectangle1 = Rectangle::new(14, 0, 16, 38);
        let mut srcrect1: &Rectangle = &rectangle1
        rectangle2 = Rectangle::new(14, 0, self.width - 28, self.height);
        let mut destrect1: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
         let mut local3: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ORDERBUTTONPRESSED);
         let mut local4: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(0, 0, 14, 38);
        let mut srcrect2: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(0, 0, 14, self.height);
        let mut destrect2: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
         let mut local5: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ORDERBUTTONPRESSED);
         let mut local6: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(30, 0, 14, 38);
        let mut srcrect3: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(self.width - 14, 0, 14, self.height);
        let mut destrect3: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
      }
      else
      {
         let mut local7: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ARROWBUTTONHIGH);
         let mut local8: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(14, 0, 7, 35);
        let mut srcrect4: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(14, 0, self.width - 28, self.height);
        let mut destrect4: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local7,  local8, srcrect4, destrect4);
         let mut local9: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ARROWBUTTONHIGH);
         let mut local10: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(0, 0, 14, 35);
        let mut srcrect5: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(0, 0, 14, self.height);
        let mut destrect5: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local9,  local10, srcrect5, destrect5);
         let mut local11: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ARROWBUTTONHIGH);
         let mut local12: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(21, 0, 14, 35);
        let mut srcrect6: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(self.width - 14, 0, 14, self.height);
        let mut destrect6: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local11,  local12, srcrect6, destrect6);
      }
      let mut x: i32 =  Math.Round( (self.width - 48) / 2.0);
      let mut y: i32 =  Math.Round( (self.height - 32) / 2.0);
      if (self.gray)
      {
         let mut local13: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
         let mut local14: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(self.iconNr * 42, 0, 42, 32);
        let mut srcrect: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x, y, 48, 32);
        let mut destrect: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2ColouredOld( local13,  local14, srcrect, destrect, 0.3f, 0.3f, 0.3f, 1f);
      }
      else
      {
         let mut local15: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
         let mut local16: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(self.iconNr * 42, 32, 42, 32);
        let mut srcrect: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x, y, 48, 32);
        let mut destrect: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local15,  local16, srcrect, destrect);
      }
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return self.OwnBitmap;
    }
  }
}
