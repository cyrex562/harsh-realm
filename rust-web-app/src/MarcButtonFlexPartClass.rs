// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MarcButtonFlexPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;

namespace WindowsApplication1
{
  pub class MarcButtonFlexPartClass : SubPartClass
  {
     OwnBitmapNr: i32;
     backbitmap: Bitmap;
     otherback: i32;
     texty: String;
     bw: i32;
     bh: i32;
     colorized: i32;

    pub fn SubDispose()
    {
      if (Information.IsNothing( this.backbitmap))
        return;
      this.backbitmap.Dispose();
      this.backbitmap = (Bitmap) null;
    }

    pub MarcButtonFlexPartClass(
      tbmpnr: i32,
      tTexty: String,
      let mut tcolorized: i32 =  0,
      tDescript: String = "",
       tBackbitmap: Bitmap = null,
      let mut bbx: i32 =  -1,
      let mut bby: i32 =  -1,
      let mut totherback: i32 =  0,
      let mut tWidth: i32 =  35,
      let mut tHeight: i32 =  35)
      : base(tWidth, tHeight)
    {
      this.OwnBitmapNr = tbmpnr;
      this.bw = tWidth;
      this.colorized = tcolorized;
      this.bh = tHeight;
      this.Descript = tDescript;
      this.texty = tTexty;
      this.otherback = totherback;
      if (Information.IsNothing( tBackbitmap))
        return;
      this.backbitmap = new Bitmap(this.OwnBitmap.Width, this.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
      this.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics Expression = Graphics.FromImage((Image) this.backbitmap);
      Expression.CompositingMode = CompositingMode.SourceCopy;
      Expression.DrawImage((Image) tBackbitmap, Rectangle::new(0, 0, this.OwnBitmap.Width, this.OwnBitmap.Height), Rectangle::new(bbx, bby, this.OwnBitmap.Width, this.OwnBitmap.Height), GraphicsUnit.Pixel);
      Expression.CompositingMode = CompositingMode.SourceOver;
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub Paint: Bitmap()
    {
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (!Information.IsNothing( this.backbitmap))
      {
        Expression.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( Expression,  this.backbitmap, 0, 0);
        Expression.CompositingMode = CompositingMode.SourceOver;
      }
      let mut nr: i32 =  0;
      num1: i32;
      num2: i32;
      if (this.otherback == 0)
      {
        nr = DrawMod.TGame.BUTTONMARC1;
        num1 = 2;
        num2 = 2;
      }
      else if (this.otherback == 1)
        nr = DrawMod.TGame.MARCBACK1;
      else if (this.otherback == 2)
        nr = DrawMod.TGame.MARCBACK2;
      else if (this.otherback == 3)
        nr = DrawMod.TGame.MARCBACK3;
      else if (this.otherback == 4)
        nr = DrawMod.TGame.MARCBACK4;
      if (this.colorized == 0)
      {
         let mut local1: &Graphics = &Expression;
        bitmap1: Bitmap = BitmapStore.GetBitmap(nr);
         let mut local2: &Bitmap = &bitmap1;
        Rectangle rectangle1 = Rectangle::new(0, 0, 8, this.bh);
        let mut srcrect1: &Rectangle = &rectangle1
        Rectangle rectangle2 = Rectangle::new(0, 0, 8, this.bh);
        let mut destrect1: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
         let mut local3: &Graphics = &Expression;
        bitmap2: Bitmap = BitmapStore.GetBitmap(nr);
         let mut local4: &Bitmap = &bitmap2;
        rectangle2 = Rectangle::new(this.bh - 8, 0, 8, this.bh);
        let mut srcrect2: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(this.bw - 8, 0, 8, this.bh);
        let mut destrect2: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
         let mut local5: &Graphics = &Expression;
        bitmap2 = BitmapStore.GetBitmap(nr);
         let mut local6: &Bitmap = &bitmap2;
        rectangle2 = Rectangle::new(8, 0, this.bh - 16, this.bh);
        let mut srcrect3: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(8, 0, this.bw - 16, this.bh);
        let mut destrect3: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
         let mut local7: &Graphics = &Expression;
        bitmap2 = BitmapStore.GetBitmap(this.OwnBitmapNr);
         let mut local8: &Bitmap = &bitmap2;
        let mut x: i32 =  num1;
        let mut y: i32 =  num2;
        DrawMod.DrawSimple( local7,  local8, x, y);
        DrawMod.DrawTextColouredMarc( Expression, this.texty, DrawMod.TGame.MarcFont4, num1 + this.bh, num2 + 4, Color.White);
      }
      else if (this.colorized == 1)
      {
         let mut local9: &Graphics = &Expression;
        bitmap3: Bitmap = BitmapStore.GetBitmap(nr);
         let mut local10: &Bitmap = &bitmap3;
        Rectangle rectangle3 = Rectangle::new(0, 0, 8, this.bh);
        let mut srcrect4: &Rectangle = &rectangle3
        Rectangle rectangle4 = Rectangle::new(0, 0, 8, this.bh);
        let mut destrect4: &Rectangle = &rectangle4
        DrawMod.DrawSimplePart2ColouredNew( local9,  local10, srcrect4, destrect4, 0.5f, 0.5f, 0.5f, 0.5f);
         let mut local11: &Graphics = &Expression;
        bitmap4: Bitmap = BitmapStore.GetBitmap(nr);
         let mut local12: &Bitmap = &bitmap4;
        rectangle3 = Rectangle::new(this.bh - 8, 0, 8, this.bh);
        let mut srcrect5: &Rectangle = &rectangle3
        rectangle4 = Rectangle::new(this.bw - 8, 0, 8, this.bh);
        let mut destrect5: &Rectangle = &rectangle4
        DrawMod.DrawSimplePart2ColouredNew( local11,  local12, srcrect5, destrect5, 0.5f, 0.5f, 0.5f, 0.5f);
         let mut local13: &Graphics = &Expression;
        bitmap4 = BitmapStore.GetBitmap(nr);
         let mut local14: &Bitmap = &bitmap4;
        rectangle3 = Rectangle::new(8, 0, this.bh - 16, this.bh);
        let mut srcrect6: &Rectangle = &rectangle3
        rectangle4 = Rectangle::new(8, 0, this.bw - 16, this.bh);
        let mut destrect6: &Rectangle = &rectangle4
        DrawMod.DrawSimplePart2ColouredNew( local13,  local14, srcrect6, destrect6, 0.5f, 0.5f, 0.5f, 0.5f);
         let mut local15: &Graphics = &Expression;
        bitmap4 = BitmapStore.GetBitmap(this.OwnBitmapNr);
         let mut local16: &Bitmap = &bitmap4;
        let mut x: i32 =  num1;
        let mut y: i32 =  num2;
        DrawMod.Draw( local15,  local16, x, y, 0.5f, 0.5f, 0.5f, 0.5f);
        DrawMod.DrawTextColouredMarc( Expression, this.texty, DrawMod.TGame.MarcFont4, num1 + this.bh, num2 + 4, Color.Gray);
      }
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return this.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (!Information.IsNothing( this.backbitmap))
      {
        Expression.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( Expression,  this.backbitmap, 0, 0);
        Expression.CompositingMode = CompositingMode.SourceOver;
      }
      let mut nr: i32 =  0;
      num1: i32;
      num2: i32;
      if (this.otherback == 0)
      {
        nr = DrawMod.TGame.BUTTONMARC1b;
        num1 = 2;
        num2 = 2;
      }
      else if (this.otherback == 1)
        nr = DrawMod.TGame.MARCBACK1B;
      else if (this.otherback == 2)
        nr = DrawMod.TGame.MARCBACK2B;
      else if (this.otherback == 3)
        nr = DrawMod.TGame.MARCBACK3B;
      else if (this.otherback == 4)
        nr = DrawMod.TGame.MARCBACK4B;
      if (this.colorized == 0)
      {
         let mut local1: &Graphics = &Expression;
        bitmap1: Bitmap = BitmapStore.GetBitmap(nr);
         let mut local2: &Bitmap = &bitmap1;
        Rectangle rectangle1 = Rectangle::new(0, 0, 8, this.bh);
        let mut srcrect1: &Rectangle = &rectangle1
        Rectangle rectangle2 = Rectangle::new(0, 0, 8, this.bh);
        let mut destrect1: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
         let mut local3: &Graphics = &Expression;
        bitmap2: Bitmap = BitmapStore.GetBitmap(nr);
         let mut local4: &Bitmap = &bitmap2;
        rectangle2 = Rectangle::new(this.bh - 8, 0, 8, this.bh);
        let mut srcrect2: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(this.bw - 8, 0, 8, this.bh);
        let mut destrect2: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
         let mut local5: &Graphics = &Expression;
        bitmap2 = BitmapStore.GetBitmap(nr);
         let mut local6: &Bitmap = &bitmap2;
        rectangle2 = Rectangle::new(8, 0, this.bh - 16, this.bh);
        let mut srcrect3: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(8, 0, this.bw - 16, this.bh);
        let mut destrect3: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
         let mut local7: &Graphics = &Expression;
        bitmap2 = BitmapStore.GetBitmap(this.OwnBitmapNr);
         let mut local8: &Bitmap = &bitmap2;
        let mut x: i32 =  num1;
        let mut y: i32 =  num2;
        DrawMod.DrawSimple( local7,  local8, x, y);
        DrawMod.DrawTextColouredMarc( Expression, this.texty, DrawMod.TGame.MarcFont4, num1 + this.bh, num2 + 4, Color.White);
      }
      else if (this.colorized == 1)
      {
         let mut local9: &Graphics = &Expression;
        bitmap3: Bitmap = BitmapStore.GetBitmap(nr);
         let mut local10: &Bitmap = &bitmap3;
        Rectangle rectangle3 = Rectangle::new(0, 0, 8, this.bh);
        let mut srcrect4: &Rectangle = &rectangle3
        Rectangle rectangle4 = Rectangle::new(0, 0, 8, this.bh);
        let mut destrect4: &Rectangle = &rectangle4
        DrawMod.DrawSimplePart2ColouredNew( local9,  local10, srcrect4, destrect4, 0.0f, 0.0f, 0.0f, 0.2f);
         let mut local11: &Graphics = &Expression;
        bitmap4: Bitmap = BitmapStore.GetBitmap(nr);
         let mut local12: &Bitmap = &bitmap4;
        rectangle3 = Rectangle::new(this.bh - 8, 0, 8, this.bh);
        let mut srcrect5: &Rectangle = &rectangle3
        rectangle4 = Rectangle::new(this.bw - 8, 0, 8, this.bh);
        let mut destrect5: &Rectangle = &rectangle4
        DrawMod.DrawSimplePart2ColouredNew( local11,  local12, srcrect5, destrect5, 0.0f, 0.0f, 0.0f, 0.2f);
         let mut local13: &Graphics = &Expression;
        bitmap4 = BitmapStore.GetBitmap(nr);
         let mut local14: &Bitmap = &bitmap4;
        rectangle3 = Rectangle::new(8, 0, 19, this.bh);
        let mut srcrect6: &Rectangle = &rectangle3
        rectangle4 = Rectangle::new(8, 0, this.bw - 16, this.bh);
        let mut destrect6: &Rectangle = &rectangle4
        DrawMod.DrawSimplePart2ColouredNew( local13,  local14, srcrect6, destrect6, 0.0f, 0.0f, 0.0f, 0.2f);
         let mut local15: &Graphics = &Expression;
        bitmap4 = BitmapStore.GetBitmap(this.OwnBitmapNr);
         let mut local16: &Bitmap = &bitmap4;
        let mut x: i32 =  num1;
        let mut y: i32 =  num2;
        DrawMod.Draw( local15,  local16, x, y, 0.0f, 0.0f, 0.0f, 0.4f);
        DrawMod.DrawTextColouredMarc( Expression, this.texty, DrawMod.TGame.MarcFont4, num1 + this.bh, num2 + 4, Color.Gray);
      }
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return this.OwnBitmap;
    }
  }
}
