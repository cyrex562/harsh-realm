// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TextButtonPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;
// usingSystem.Drawing.Text;

namespace WindowsApplication1
{
  pub class TextButtonPartClass : SubPartClass
  {
     bool overrule;
     buttext: String;
     width: i32;
     height: i32;
     ourfont: Font;
     ourfont2: Font;
     backbitmap: Bitmap;
     bool inactive;
     bool red;
     bool tuseshadow;
     bool marcStyle;
     extraS: String;
     bool udsButton;
     udsButtonSubType: i32;
     bool useOverruleCol;
     overruleCol: Color;

    pub fn Click(x: i32, y: i32, let mut b: i32 = 1) -> i32
    {
      if (DrawMod.TGame.EmpireStyle)
        SoundMod.PlayAWave(DrawMod.TGame.AppPath + "sound/interface/click.wav",  DrawMod.TGame.EditObj);
      num: i32;
      return num;
    }

    pub TextButtonPartClass(
      buttontext: String,
      twidth: i32,
      tDescript: String = "",
       tBackbitmap: Bitmap = null,
      let mut bbx: i32 = -1,
      let mut bby: i32 = -1,
      bool tinactive = false,
      bool tred = false,
      let mut theight: i32 = 35,
      let mut tfontsize: i32 = 13,
      usefont: Font = null,
      bool useshadow = false,
      bool tMarcStyle = false,
      textras: String = "",
      tfont2: Font = null,
      bool tudsButton = false,
      let mut tudsButtonSubType: i32 = 0,
      let mut toverrulered: i32 = 0,
      let mut toverrulegreen: i32 = 0,
      let mut toverruleblue: i32 = 0)
      : base(twidth, theight)
    {
      if (tfontsize > -1)
        self.ourfont = Font::new(DrawMod.TGame.FontCol.Families[1],  tfontsize, FontStyle.Bold, GraphicsUnit.Pixel);
      self.udsButton = tudsButton;
      self.udsButtonSubType = tudsButtonSubType;
      self.overrule = false;
      self.Descript = tDescript;
      if (!Information.IsNothing( tBackbitmap))
      {
        self.backbitmap = new Bitmap(self.OwnBitmap.Width, self.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
        self.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics Expression = Graphics.FromImage((Image) self.backbitmap);
        Expression.CompositingMode = CompositingMode.SourceCopy;
        Expression.DrawImage((Image) tBackbitmap, Rectangle::new(0, 0, self.OwnBitmap.Width, self.OwnBitmap.Height), Rectangle::new(bbx, bby, self.OwnBitmap.Width, self.OwnBitmap.Height), GraphicsUnit.Pixel);
        Expression.CompositingMode = CompositingMode.SourceOver;
        if (!Information.IsNothing( Expression))
          Expression.Dispose();
      }
      self.buttext = buttontext;
      self.width = twidth;
      self.height = theight;
      self.inactive = tinactive;
      self.red = tred;
      self.extraS = textras;
      self.marcStyle = tMarcStyle;
      self.tuseshadow = useshadow;
      if (!Information.IsNothing( usefont))
        self.ourfont = usefont;
      if (!Information.IsNothing( tfont2))
        self.ourfont2 = tfont2;
      if (!(toverruleblue > 0 | toverrulegreen > 0 | toverrulered > 0))
        return;
      self.overruleCol = Color.FromArgb( byte.MaxValue, toverrulered, toverrulegreen, toverruleblue);
      self.useOverruleCol = true;
    }

    pub fn SubDispose()
    {
      if (Information.IsNothing( self.backbitmap))
        return;
      self.backbitmap.Dispose();
      self.backbitmap = (Bitmap) null;
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      if (!Information.IsNothing( self.backbitmap))
      {
        Expression.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( Expression,  self.backbitmap, 0, 0);
        Expression.CompositingMode = CompositingMode.SourceOver;
      }
      Expression.SmoothingMode = SmoothingMode.AntiAlias;
      Expression.TextRenderingHint = TextRenderingHint.AntiAlias;
      Expression.TextContrast = 1;
      if (self.udsButton)
      {
        if (self.udsButtonSubType == 3)
        {
          if (self.inactive)
          {
            DrawMod.DrawBlock( Expression, 4, 3, self.width - 12, self.height, 0, 0, 0, 96);
          }
          else
          {
            DrawMod.DrawBlock( Expression, 4, 3, self.width - 12, self.height, 0, 0, 0, 24);
            DrawMod.DrawRectangle( Expression, 4, 3, self.width - 12, self.height - 6, 0, 0, 0, 64, 4);
          }
        }
        else if (self.udsButtonSubType < 2 | self.udsButtonSubType == 4)
        {
          if (self.inactive)
          {
            DrawMod.DrawBlock( Expression, 4, 3, self.width - 12, self.height - 10, 0, 0, 0, 96);
          }
          else
          {
            DrawMod.DrawBlock( Expression, 4, 3, self.width - 12, self.height - 10, 0, 0, 0, 24);
            DrawMod.DrawRectangle( Expression, 4, 3, self.width - 12, self.height - 10, 0, 0, 0, 64, 4);
          }
        }
        else if (self.udsButtonSubType == 2)
        {
          if (self.height == 35)
          {
             let mut local1: &Graphics = &Expression;
            bitmap1: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local2: &Bitmap = &bitmap1;
            Rectangle rectangle1 = Rectangle::new(15, 0, 205, 35);
            let mut srcrect1: &Rectangle = &rectangle1
            Rectangle rectangle2 = Rectangle::new(15, 0, self.width - 30, self.height);
            let mut destrect1: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
             let mut local3: &Graphics = &Expression;
            bitmap2: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local4: &Bitmap = &bitmap2;
            rectangle2 = Rectangle::new(0, 0, 15, 35);
            let mut srcrect2: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, 0, 15, self.height);
            let mut destrect2: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
             let mut local5: &Graphics = &Expression;
            bitmap3: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local6: &Bitmap = &bitmap3;
            rectangle2 = Rectangle::new(220, 0, 15, 35);
            let mut srcrect3: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(self.width - 15, 0, 15, self.height);
            let mut destrect3: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
          }
          else
          {
             let mut local7: &Graphics = &Expression;
            bitmap4: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local8: &Bitmap = &bitmap4;
            Rectangle rectangle3 = Rectangle::new(7, 5, 222, 22);
            let mut srcrect4: &Rectangle = &rectangle3
            Rectangle rectangle4 = Rectangle::new(7, 5, self.width - 14, self.height - 8);
            let mut destrect4: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local7,  local8, srcrect4, destrect4);
             let mut local9: &Graphics = &Expression;
            bitmap5: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local10: &Bitmap = &bitmap5;
            rectangle3 = Rectangle::new(0, 5, 7, 22);
            let mut srcrect5: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(0, 5, 7, self.height - 8);
            let mut destrect5: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local9,  local10, srcrect5, destrect5);
             let mut local11: &Graphics = &Expression;
            bitmap6: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local12: &Bitmap = &bitmap6;
            rectangle3 = Rectangle::new(224, 5, 12, 22);
            let mut srcrect6: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(self.width - 12, 5, 12, self.height - 8);
            let mut destrect6: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local11,  local12, srcrect6, destrect6);
             let mut local13: &Graphics = &Expression;
            bitmap7: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local14: &Bitmap = &bitmap7;
            rectangle3 = Rectangle::new(7, 0, 222, 5);
            let mut srcrect7: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(7, 0, self.width - 14, 5);
            let mut destrect7: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local13,  local14, srcrect7, destrect7);
             let mut local15: &Graphics = &Expression;
            bitmap7 = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local16: &Bitmap = &bitmap7;
            rectangle3 = Rectangle::new(0, 0, 7, 5);
            let mut srcrect8: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(0, 0, 7, 5);
            let mut destrect8: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local15,  local16, srcrect8, destrect8);
             let mut local17: &Graphics = &Expression;
            bitmap7 = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local18: &Bitmap = &bitmap7;
            rectangle3 = Rectangle::new(224, 0, 12, 5);
            let mut srcrect9: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(self.width - 12, 0, 12, 5);
            let mut destrect9: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local17,  local18, srcrect9, destrect9);
             let mut local19: &Graphics = &Expression;
            bitmap7 = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local20: &Bitmap = &bitmap7;
            rectangle3 = Rectangle::new(7, 28, 222, 7);
            let mut srcrect10: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(7, self.height - 4, self.width - 14, 4);
            let mut destrect10: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local19,  local20, srcrect10, destrect10);
             let mut local21: &Graphics = &Expression;
            bitmap7 = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local22: &Bitmap = &bitmap7;
            rectangle3 = Rectangle::new(0, 28, 7, 7);
            let mut srcrect11: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(0, self.height - 4, 7, 4);
            let mut destrect11: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local21,  local22, srcrect11, destrect11);
             let mut local23: &Graphics = &Expression;
            bitmap7 = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONG);
             let mut local24: &Bitmap = &bitmap7;
            rectangle3 = Rectangle::new(224, 28, 12, 7);
            let mut srcrect12: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(self.width - 12, self.height - 4, 12, 4);
            let mut destrect12: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local23,  local24, srcrect12, destrect12);
          }
        }
      }
      else if (self.marcStyle)
      {
        if (self.width > 39 & DrawMod.TGame.Data.Product == 7)
        {
           let mut local25: &Graphics = &Expression;
          bitmap8: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
           let mut local26: &Bitmap = &bitmap8;
          Rectangle rectangle5 = Rectangle::new(14, 0, 197, 35);
          let mut srcrect13: &Rectangle = &rectangle5
          Rectangle rectangle6 = Rectangle::new(14, 0, self.width - 28, self.height);
          let mut destrect13: &Rectangle = &rectangle6
          DrawMod.DrawSimplePart2( local25,  local26, srcrect13, destrect13);
           let mut local27: &Graphics = &Expression;
          bitmap9: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
           let mut local28: &Bitmap = &bitmap9;
          rectangle5 = Rectangle::new(0, 0, 14, 35);
          let mut srcrect14: &Rectangle = &rectangle5
          rectangle6 = Rectangle::new(0, 0, 14, self.height);
          let mut destrect14: &Rectangle = &rectangle6
          DrawMod.DrawSimplePart2( local27,  local28, srcrect14, destrect14);
           let mut local29: &Graphics = &Expression;
          bitmap10: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
           let mut local30: &Bitmap = &bitmap10;
          rectangle5 = Rectangle::new(211, 0, 24, 35);
          let mut srcrect15: &Rectangle = &rectangle5
          rectangle6 = Rectangle::new(self.width - 24, 0, 24, self.height);
          let mut destrect15: &Rectangle = &rectangle6
          DrawMod.DrawSimplePart2( local29,  local30, srcrect15, destrect15);
        }
        else
        {
           let mut local31: &Graphics = &Expression;
          bitmap11: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONMARC1);
           let mut local32: &Bitmap = &bitmap11;
          Rectangle rectangle7 = Rectangle::new(7, 0, 7, 35);
          let mut srcrect16: &Rectangle = &rectangle7
          Rectangle rectangle8 = Rectangle::new(7, 0, self.width - 14, self.height);
          let mut destrect16: &Rectangle = &rectangle8
          DrawMod.DrawSimplePart2( local31,  local32, srcrect16, destrect16);
           let mut local33: &Graphics = &Expression;
          bitmap12: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONMARC1);
           let mut local34: &Bitmap = &bitmap12;
          rectangle7 = Rectangle::new(0, 0, 7, 35);
          let mut srcrect17: &Rectangle = &rectangle7
          rectangle8 = Rectangle::new(0, 0, 7, self.height);
          let mut destrect17: &Rectangle = &rectangle8
          DrawMod.DrawSimplePart2( local33,  local34, srcrect17, destrect17);
           let mut local35: &Graphics = &Expression;
          bitmap13: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONMARC1);
           let mut local36: &Bitmap = &bitmap13;
          rectangle7 = Rectangle::new(29, 0, 7, 35);
          let mut srcrect18: &Rectangle = &rectangle7
          rectangle8 = Rectangle::new(self.width - 7, 0, 7, self.height);
          let mut destrect18: &Rectangle = &rectangle8
          DrawMod.DrawSimplePart2( local35,  local36, srcrect18, destrect18);
        }
      }
      else if (self.width > 39)
      {
         let mut local37: &Graphics = &Expression;
        bitmap14: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
         let mut local38: &Bitmap = &bitmap14;
        Rectangle rectangle9 = Rectangle::new(7, 5, 222, 22);
        let mut srcrect19: &Rectangle = &rectangle9
        Rectangle rectangle10 = Rectangle::new(7, 5, self.width - 14, self.height - 8);
        let mut destrect19: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local37,  local38, srcrect19, destrect19);
         let mut local39: &Graphics = &Expression;
        bitmap15: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
         let mut local40: &Bitmap = &bitmap15;
        rectangle9 = Rectangle::new(0, 5, 7, 22);
        let mut srcrect20: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(0, 5, 7, self.height - 8);
        let mut destrect20: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local39,  local40, srcrect20, destrect20);
         let mut local41: &Graphics = &Expression;
        bitmap16: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
         let mut local42: &Bitmap = &bitmap16;
        rectangle9 = Rectangle::new(224, 5, 12, 22);
        let mut srcrect21: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(self.width - 12, 5, 12, self.height - 8);
        let mut destrect21: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local41,  local42, srcrect21, destrect21);
         let mut local43: &Graphics = &Expression;
        bitmap17: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
         let mut local44: &Bitmap = &bitmap17;
        rectangle9 = Rectangle::new(7, 0, 222, 5);
        let mut srcrect22: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(7, 0, self.width - 14, 5);
        let mut destrect22: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local43,  local44, srcrect22, destrect22);
         let mut local45: &Graphics = &Expression;
        bitmap17 = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
         let mut local46: &Bitmap = &bitmap17;
        rectangle9 = Rectangle::new(0, 0, 7, 5);
        let mut srcrect23: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(0, 0, 7, 5);
        let mut destrect23: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local45,  local46, srcrect23, destrect23);
         let mut local47: &Graphics = &Expression;
        bitmap17 = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
         let mut local48: &Bitmap = &bitmap17;
        rectangle9 = Rectangle::new(224, 0, 12, 5);
        let mut srcrect24: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(self.width - 12, 0, 12, 5);
        let mut destrect24: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local47,  local48, srcrect24, destrect24);
         let mut local49: &Graphics = &Expression;
        bitmap17 = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
         let mut local50: &Bitmap = &bitmap17;
        rectangle9 = Rectangle::new(7, 28, 222, 7);
        let mut srcrect25: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(7, self.height - 4, self.width - 14, 4);
        let mut destrect25: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local49,  local50, srcrect25, destrect25);
         let mut local51: &Graphics = &Expression;
        bitmap17 = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
         let mut local52: &Bitmap = &bitmap17;
        rectangle9 = Rectangle::new(0, 28, 7, 7);
        let mut srcrect26: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(0, self.height - 4, 7, 4);
        let mut destrect26: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local51,  local52, srcrect26, destrect26);
         let mut local53: &Graphics = &Expression;
        bitmap17 = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONG);
         let mut local54: &Bitmap = &bitmap17;
        rectangle9 = Rectangle::new(224, 28, 12, 7);
        let mut srcrect27: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(self.width - 12, self.height - 4, 12, 4);
        let mut destrect27: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local53,  local54, srcrect27, destrect27);
      }
      else
      {
         let mut local55: &Graphics = &Expression;
        bitmap18: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONSTEVE1);
         let mut local56: &Bitmap = &bitmap18;
        Rectangle rectangle11 = Rectangle::new(7, 0, 7, 35);
        let mut srcrect28: &Rectangle = &rectangle11
        Rectangle rectangle12 = Rectangle::new(7, 0, self.width - 14, self.height);
        let mut destrect28: &Rectangle = &rectangle12
        DrawMod.DrawSimplePart2( local55,  local56, srcrect28, destrect28);
         let mut local57: &Graphics = &Expression;
        bitmap19: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONSTEVE1);
         let mut local58: &Bitmap = &bitmap19;
        rectangle11 = Rectangle::new(0, 0, 7, 35);
        let mut srcrect29: &Rectangle = &rectangle11
        rectangle12 = Rectangle::new(0, 0, 7, self.height);
        let mut destrect29: &Rectangle = &rectangle12
        DrawMod.DrawSimplePart2( local57,  local58, srcrect29, destrect29);
         let mut local59: &Graphics = &Expression;
        bitmap20: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONSTEVE1);
         let mut local60: &Bitmap = &bitmap20;
        rectangle11 = Rectangle::new(29, 0, 7, 35);
        let mut srcrect30: &Rectangle = &rectangle11
        rectangle12 = Rectangle::new(self.width - 7, 0, 7, self.height);
        let mut destrect30: &Rectangle = &rectangle12
        DrawMod.DrawSimplePart2( local59,  local60, srcrect30, destrect30);
      }
      if (!self.udsButton)
      {
        if (self.inactive & !self.marcStyle)
          DrawMod.DrawBlock( Expression, 0, 0, self.width, self.height, 0, 0, 0, 128);
        else if (self.inactive & self.marcStyle)
        {
          if (DrawMod.TGame.Data.Product >= 6 & self.red)
            DrawMod.DrawBlockGradient2( Expression, -1, -1, self.width - 6,  Math.Round( self.height - (3.0 +  Math.Max(0, self.height - 40) / 7.0)), Color.FromArgb(64, 155, 25, 0), Color.FromArgb(164, 155, 25, 0));
          else
            DrawMod.DrawBlockGradient2( Expression, 2, 2, self.width - 6,  Math.Round( self.height - (3.0 +  Math.Max(0, self.height - 40) / 7.0)), Color.FromArgb(128, 0, 0, 0), Color.FromArgb(196, 0, 0, 0));
        }
      }
      c: Color = Color.White;
      if (DrawMod.TGame.Data.Product == 7 & self.inactive)
        c = Color.Gray;
      if (self.marcStyle & self.red)
        c = DrawMod.TGame.Data.Product != 7 ? Color.FromArgb( byte.MaxValue,  byte.MaxValue, 210, 210) : Color.FromArgb( byte.MaxValue,  byte.MaxValue, 60, 60);
      if (self.marcStyle & self.useOverruleCol)
        c = self.overruleCol;
      if (self.udsButton)
      {
        c = Color.Black;
        if (self.udsButtonSubType == 2)
          c = Color.White;
      }
      SizeF sizeF2;
      num: i32;
      y1: i32;
      if (self.udsButton)
      {
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        num =  Math.Round(( (self.width - 6) -  sizeF2.Width) / 2.0);
        y1 =  Math.Round(( (self.height + 2) -  sizeF2.Height) / 2.0 - 1.0);
        if (self.udsButtonSubType == 3)
          y1 += 2;
        if (self.inactive)
          c = Color.FromArgb( Math.Round( c.A / 2.0),  c.R,  c.G,  c.B);
        DrawMod.DrawTextColouredNicely( Expression, self.buttext, self.ourfont, num, y1, c);
      }
      else if (!self.marcStyle & !self.tuseshadow)
      {
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        num =  Math.Round(( (self.width - 6) -  sizeF2.Width) / 2.0);
        y1 =  Math.Round(( (self.height + 2) -  sizeF2.Height) / 2.0 - 1.0);
        DrawMod.DrawTextVic3( Expression, self.buttext, self.ourfont, num, y1, DrawMod.TGame.VicColor2, DrawMod.TGame.VicColor1Shade);
      }
      else if (self.extraS.Length > 0)
      {
        SizeF sizeF3 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x1: i32 =  Math.Round(2.0 + ( self.width -  sizeF3.Width) / 2.0);
        let mut y2: i32 =  Math.Round(2.0 + ( self.height -  sizeF3.Height) / 2.0 - 8.0);
        DrawMod.DrawTextColouredFuzzy( Expression, self.buttext, self.ourfont, x1, y2, Color.FromArgb(200, 0, 0, 0));
        SizeF sizeF4 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x2: i32 =  Math.Round(( self.width -  sizeF4.Width) / 2.0);
        let mut y3: i32 =  Math.Round(( self.height -  sizeF4.Height) / 2.0 - 8.0);
        DrawMod.DrawTextColouredNicely( Expression, self.buttext, self.ourfont, x2, y3, c);
        SizeF sizeF5 = Expression.MeasureString(self.extraS, self.ourfont2);
        let mut x3: i32 =  Math.Round(2.0 + ( self.width -  sizeF5.Width) / 2.0);
        let mut y4: i32 =  Math.Round(2.0 + ( self.height -  sizeF5.Height) / 2.0 + 8.0);
        DrawMod.DrawTextColouredFuzzy( Expression, self.extraS, self.ourfont2, x3, y4, Color.FromArgb(200, 0, 0, 0));
        sizeF2 = Expression.MeasureString(self.extraS, self.ourfont2);
        num =  Math.Round(( self.width -  sizeF2.Width) / 2.0);
        y1 =  Math.Round(( self.height -  sizeF2.Height) / 2.0 + 8.0);
        DrawMod.DrawTextColouredNicely( Expression, self.extraS, self.ourfont2, num, y1, c);
      }
      else if (self.tuseshadow)
      {
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x: i32 =  Math.Round(( self.width -  sizeF2.Width) / 2.0);
        let mut y5: i32 =  Math.Round(2.0 + ( self.height -  sizeF2.Height) / 2.0);
        DrawMod.DrawTextColouredFuzzy( Expression, self.buttext, self.ourfont, x, y5, Color.FromArgb(200, 0, 0, 0));
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        num =  Math.Round(( self.width -  sizeF2.Width) / 2.0 - 2.0);
        y1 =  Math.Round(0.0 + ( self.height -  sizeF2.Height) / 2.0);
        DrawMod.DrawTextColouredNicely( Expression, self.buttext, self.ourfont, num, y1, c);
      }
      else
      {
        SizeF sizeF6 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x4: i32 =  Math.Round(( self.width -  sizeF6.Width) / 2.0);
        let mut y6: i32 =  Math.Round(( self.height -  sizeF6.Height) / 2.0 - 1.0);
        DrawMod.DrawTextColoured( Expression, self.buttext, self.ourfont, x4, y6, Color.Black);
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x5: i32 =  Math.Round(( self.width -  sizeF2.Width) / 2.0);
        let mut y7: i32 =  Math.Round(( self.height -  sizeF2.Height) / 2.0 + 1.0);
        DrawMod.DrawTextColoured( Expression, self.buttext, self.ourfont, x5, y7, Color.Black);
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x6: i32 =  Math.Round(( self.width -  sizeF2.Width) / 2.0 + 1.0);
        let mut y8: i32 =  Math.Round(( self.height -  sizeF2.Height) / 2.0);
        DrawMod.DrawTextColoured( Expression, self.buttext, self.ourfont, x6, y8, Color.Black);
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x7: i32 =  Math.Round(( self.width -  sizeF2.Width) / 2.0 - 1.0);
        let mut y9: i32 =  Math.Round(( self.height -  sizeF2.Height) / 2.0);
        DrawMod.DrawTextColoured( Expression, self.buttext, self.ourfont, x7, y9, Color.Black);
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        num =  Math.Round(( self.width -  sizeF2.Width) / 2.0);
        y1 =  Math.Round(( self.height -  sizeF2.Height) / 2.0);
        DrawMod.DrawTextColoured( Expression, self.buttext, self.ourfont, num, y1, c);
      }
      if (!self.marcStyle && self.red)
      {
        if (self.height == 26)
          DrawMod.drawLine( Expression, num,  Math.Round( y1 +  sizeF2.Height - 1.0),  Math.Round( ( num + sizeF2.Width)),  Math.Round( y1 +  sizeF2.Height - 1.0),  DrawMod.TGame.viccolor7.R,  DrawMod.TGame.viccolor7.G,  DrawMod.TGame.viccolor7.B,  DrawMod.TGame.viccolor7.A);
        else
          DrawMod.drawLine( Expression, num,  Math.Round( y1 +  sizeF2.Height + 2.0),  Math.Round( ( num + sizeF2.Width)),  Math.Round( y1 +  sizeF2.Height + 2.0),  DrawMod.TGame.viccolor7.R,  DrawMod.TGame.viccolor7.G,  DrawMod.TGame.viccolor7.B,  DrawMod.TGame.viccolor7.A);
      }
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return self.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      if (self.inactive)
        return self.Paint();
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      if (!Information.IsNothing( self.backbitmap))
        DrawMod.DrawSimple( Expression,  self.backbitmap, 0, 0);
      Expression.SmoothingMode = SmoothingMode.AntiAlias;
      Expression.TextRenderingHint = TextRenderingHint.AntiAlias;
      Expression.TextContrast = 1;
      if (self.udsButton)
      {
        if (self.udsButtonSubType == 3 | self.udsButtonSubType == 4)
        {
          DrawMod.DrawBlock( Expression, 3, 2, self.width - 12, self.height,  byte.MaxValue, 225, 225, 24);
          DrawMod.DrawRectangle( Expression, 3, 2, self.width - 12, self.height - 6, 0, 0, 0, 32, 4);
        }
        else if (self.udsButtonSubType < 2 | self.udsButtonSubType == 3)
        {
          DrawMod.DrawBlock( Expression, 3, 2, self.width - 12, self.height - 6,  byte.MaxValue, 225, 225, 24);
          DrawMod.DrawRectangle( Expression, 3, 2, self.width - 12, self.height - 6, 0, 0, 0, 32, 4);
        }
        else if (self.udsButtonSubType == 2)
        {
          if (self.height == 35)
          {
             let mut local1: &Graphics = &Expression;
            bitmap1: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local2: &Bitmap = &bitmap1;
            Rectangle rectangle1 = Rectangle::new(15, 0, 205, 35);
            let mut srcrect1: &Rectangle = &rectangle1
            Rectangle rectangle2 = Rectangle::new(15, 0, self.width - 30, self.height);
            let mut destrect1: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
             let mut local3: &Graphics = &Expression;
            bitmap2: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local4: &Bitmap = &bitmap2;
            rectangle2 = Rectangle::new(0, 0, 15, 35);
            let mut srcrect2: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, 0, 15, self.height);
            let mut destrect2: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
             let mut local5: &Graphics = &Expression;
            bitmap3: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local6: &Bitmap = &bitmap3;
            rectangle2 = Rectangle::new(220, 0, 15, 35);
            let mut srcrect3: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(self.width - 15, 0, 15, self.height);
            let mut destrect3: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
          }
          else
          {
             let mut local7: &Graphics = &Expression;
            bitmap4: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local8: &Bitmap = &bitmap4;
            Rectangle rectangle3 = Rectangle::new(7, 5, 222, 22);
            let mut srcrect4: &Rectangle = &rectangle3
            Rectangle rectangle4 = Rectangle::new(7, 5, self.width - 14, self.height - 8);
            let mut destrect4: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local7,  local8, srcrect4, destrect4);
             let mut local9: &Graphics = &Expression;
            bitmap5: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local10: &Bitmap = &bitmap5;
            rectangle3 = Rectangle::new(0, 5, 7, 22);
            let mut srcrect5: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(0, 5, 7, self.height - 8);
            let mut destrect5: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local9,  local10, srcrect5, destrect5);
             let mut local11: &Graphics = &Expression;
            bitmap6: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local12: &Bitmap = &bitmap6;
            rectangle3 = Rectangle::new(224, 5, 12, 22);
            let mut srcrect6: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(self.width - 12, 5, 12, self.height - 8);
            let mut destrect6: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local11,  local12, srcrect6, destrect6);
             let mut local13: &Graphics = &Expression;
            bitmap7: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local14: &Bitmap = &bitmap7;
            rectangle3 = Rectangle::new(7, 0, 222, 5);
            let mut srcrect7: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(7, 0, self.width - 14, 5);
            let mut destrect7: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local13,  local14, srcrect7, destrect7);
             let mut local15: &Graphics = &Expression;
            bitmap8: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local16: &Bitmap = &bitmap8;
            rectangle3 = Rectangle::new(0, 0, 7, 5);
            let mut srcrect8: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(0, 0, 7, 5);
            let mut destrect8: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local15,  local16, srcrect8, destrect8);
             let mut local17: &Graphics = &Expression;
            bitmap9: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local18: &Bitmap = &bitmap9;
            rectangle3 = Rectangle::new(224, 0, 12, 5);
            let mut srcrect9: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(self.width - 12, 0, 12, 5);
            let mut destrect9: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local17,  local18, srcrect9, destrect9);
             let mut local19: &Graphics = &Expression;
            bitmap10: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local20: &Bitmap = &bitmap10;
            rectangle3 = Rectangle::new(7, 28, 222, 7);
            let mut srcrect10: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(7, self.height - 4, self.width - 14, 4);
            let mut destrect10: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local19,  local20, srcrect10, destrect10);
             let mut local21: &Graphics = &Expression;
            bitmap11: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local22: &Bitmap = &bitmap11;
            rectangle3 = Rectangle::new(0, 28, 7, 7);
            let mut srcrect11: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(0, self.height - 4, 7, 4);
            let mut destrect11: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local21,  local22, srcrect11, destrect11);
             let mut local23: &Graphics = &Expression;
            bitmap12: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.UDSBUT2LONGHIGH);
             let mut local24: &Bitmap = &bitmap12;
            rectangle3 = Rectangle::new(224, 28, 12, 7);
            let mut srcrect12: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(self.width - 12, self.height - 4, 12, 4);
            let mut destrect12: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local23,  local24, srcrect12, destrect12);
          }
        }
      }
      else if (self.marcStyle)
      {
        if (self.width > 39 & DrawMod.TGame.Data.Product == 7)
        {
           let mut local25: &Graphics = &Expression;
          bitmap13: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
           let mut local26: &Bitmap = &bitmap13;
          Rectangle rectangle5 = Rectangle::new(14, 0, 197, 35);
          let mut srcrect13: &Rectangle = &rectangle5
          Rectangle rectangle6 = Rectangle::new(14, 0, self.width - 28, self.height);
          let mut destrect13: &Rectangle = &rectangle6
          DrawMod.DrawSimplePart2( local25,  local26, srcrect13, destrect13);
           let mut local27: &Graphics = &Expression;
          bitmap14: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
           let mut local28: &Bitmap = &bitmap14;
          rectangle5 = Rectangle::new(0, 0, 14, 35);
          let mut srcrect14: &Rectangle = &rectangle5
          rectangle6 = Rectangle::new(0, 0, 14, self.height);
          let mut destrect14: &Rectangle = &rectangle6
          DrawMod.DrawSimplePart2( local27,  local28, srcrect14, destrect14);
           let mut local29: &Graphics = &Expression;
          bitmap15: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
           let mut local30: &Bitmap = &bitmap15;
          rectangle5 = Rectangle::new(211, 0, 24, 35);
          let mut srcrect15: &Rectangle = &rectangle5
          rectangle6 = Rectangle::new(self.width - 24, 0, 24, self.height);
          let mut destrect15: &Rectangle = &rectangle6
          DrawMod.DrawSimplePart2( local29,  local30, srcrect15, destrect15);
        }
        else
        {
           let mut local31: &Graphics = &Expression;
          bitmap16: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONMARC1b);
           let mut local32: &Bitmap = &bitmap16;
          Rectangle rectangle7 = Rectangle::new(7, 0, 7, 35);
          let mut srcrect16: &Rectangle = &rectangle7
          Rectangle rectangle8 = Rectangle::new(7, 0, self.width - 14, self.height);
          let mut destrect16: &Rectangle = &rectangle8
          DrawMod.DrawSimplePart2( local31,  local32, srcrect16, destrect16);
           let mut local33: &Graphics = &Expression;
          bitmap17: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONMARC1b);
           let mut local34: &Bitmap = &bitmap17;
          rectangle7 = Rectangle::new(0, 0, 7, 35);
          let mut srcrect17: &Rectangle = &rectangle7
          rectangle8 = Rectangle::new(0, 0, 7, self.height);
          let mut destrect17: &Rectangle = &rectangle8
          DrawMod.DrawSimplePart2( local33,  local34, srcrect17, destrect17);
           let mut local35: &Graphics = &Expression;
          bitmap18: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONMARC1b);
           let mut local36: &Bitmap = &bitmap18;
          rectangle7 = Rectangle::new(29, 0, 7, 35);
          let mut srcrect18: &Rectangle = &rectangle7
          rectangle8 = Rectangle::new(self.width - 7, 0, 7, self.height);
          let mut destrect18: &Rectangle = &rectangle8
          DrawMod.DrawSimplePart2( local35,  local36, srcrect18, destrect18);
        }
      }
      else if (self.width > 39)
      {
         let mut local37: &Graphics = &Expression;
        bitmap19: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
         let mut local38: &Bitmap = &bitmap19;
        Rectangle rectangle9 = Rectangle::new(7, 5, 222, 22);
        let mut srcrect19: &Rectangle = &rectangle9
        Rectangle rectangle10 = Rectangle::new(7, 5, self.width - 14, self.height - 8);
        let mut destrect19: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local37,  local38, srcrect19, destrect19);
         let mut local39: &Graphics = &Expression;
        bitmap20: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
         let mut local40: &Bitmap = &bitmap20;
        rectangle9 = Rectangle::new(0, 5, 7, 22);
        let mut srcrect20: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(0, 5, 7, self.height - 8);
        let mut destrect20: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local39,  local40, srcrect20, destrect20);
         let mut local41: &Graphics = &Expression;
        bitmap21: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
         let mut local42: &Bitmap = &bitmap21;
        rectangle9 = Rectangle::new(224, 5, 12, 22);
        let mut srcrect21: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(self.width - 12, 5, 12, self.height - 8);
        let mut destrect21: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local41,  local42, srcrect21, destrect21);
         let mut local43: &Graphics = &Expression;
        bitmap22: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
         let mut local44: &Bitmap = &bitmap22;
        rectangle9 = Rectangle::new(7, 0, 222, 5);
        let mut srcrect22: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(7, 0, self.width - 14, 5);
        let mut destrect22: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local43,  local44, srcrect22, destrect22);
         let mut local45: &Graphics = &Expression;
        bitmap23: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
         let mut local46: &Bitmap = &bitmap23;
        rectangle9 = Rectangle::new(0, 0, 7, 5);
        let mut srcrect23: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(0, 0, 7, 5);
        let mut destrect23: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local45,  local46, srcrect23, destrect23);
         let mut local47: &Graphics = &Expression;
        bitmap24: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
         let mut local48: &Bitmap = &bitmap24;
        rectangle9 = Rectangle::new(224, 0, 12, 5);
        let mut srcrect24: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(self.width - 12, 0, 12, 5);
        let mut destrect24: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local47,  local48, srcrect24, destrect24);
         let mut local49: &Graphics = &Expression;
        bitmap25: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
         let mut local50: &Bitmap = &bitmap25;
        rectangle9 = Rectangle::new(7, 28, 222, 7);
        let mut srcrect25: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(7, self.height - 4, self.width - 14, 4);
        let mut destrect25: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local49,  local50, srcrect25, destrect25);
         let mut local51: &Graphics = &Expression;
        bitmap26: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
         let mut local52: &Bitmap = &bitmap26;
        rectangle9 = Rectangle::new(0, 28, 7, 7);
        let mut srcrect26: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(0, self.height - 4, 7, 4);
        let mut destrect26: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local51,  local52, srcrect26, destrect26);
         let mut local53: &Graphics = &Expression;
        bitmap27: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONLONGHIGH);
         let mut local54: &Bitmap = &bitmap27;
        rectangle9 = Rectangle::new(224, 28, 12, 7);
        let mut srcrect27: &Rectangle = &rectangle9
        rectangle10 = Rectangle::new(self.width - 12, self.height - 4, 12, 4);
        let mut destrect27: &Rectangle = &rectangle10
        DrawMod.DrawSimplePart2( local53,  local54, srcrect27, destrect27);
      }
      else
      {
         let mut local55: &Graphics = &Expression;
        bitmap28: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONSTEVE1b);
         let mut local56: &Bitmap = &bitmap28;
        Rectangle rectangle11 = Rectangle::new(7, 0, 7, 35);
        let mut srcrect28: &Rectangle = &rectangle11
        Rectangle rectangle12 = Rectangle::new(7, 0, self.width - 14, self.height);
        let mut destrect28: &Rectangle = &rectangle12
        DrawMod.DrawSimplePart2( local55,  local56, srcrect28, destrect28);
         let mut local57: &Graphics = &Expression;
        bitmap29: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONSTEVE1b);
         let mut local58: &Bitmap = &bitmap29;
        rectangle11 = Rectangle::new(0, 0, 7, 35);
        let mut srcrect29: &Rectangle = &rectangle11
        rectangle12 = Rectangle::new(0, 0, 7, self.height);
        let mut destrect29: &Rectangle = &rectangle12
        DrawMod.DrawSimplePart2( local57,  local58, srcrect29, destrect29);
         let mut local59: &Graphics = &Expression;
        bitmap30: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONSTEVE1b);
         let mut local60: &Bitmap = &bitmap30;
        rectangle11 = Rectangle::new(29, 0, 7, 35);
        let mut srcrect30: &Rectangle = &rectangle11
        rectangle12 = Rectangle::new(self.width - 7, 0, 7, self.height);
        let mut destrect30: &Rectangle = &rectangle12
        DrawMod.DrawSimplePart2( local59,  local60, srcrect30, destrect30);
      }
      c: Color = Color.White;
      if (self.marcStyle & self.red)
        c = DrawMod.TGame.Data.Product != 7 ? Color.FromArgb( byte.MaxValue,  byte.MaxValue, 210, 210) : Color.FromArgb( byte.MaxValue,  byte.MaxValue, 110, 110);
      if (self.marcStyle & self.useOverruleCol)
        c = self.overruleCol;
      if (self.udsButton)
        c = Color.Red;
      SizeF sizeF2;
      num: i32;
      y1: i32;
      if (self.udsButton)
      {
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        num =  Math.Round(( (self.width - 6) -  sizeF2.Width) / 2.0);
        y1 =  Math.Round(( (self.height + 2) -  sizeF2.Height) / 2.0 - 1.0);
        if (self.udsButtonSubType == 3)
          y1 += 2;
        DrawMod.DrawTextColouredNicely( Expression, self.buttext, self.ourfont, num, y1, c);
      }
      else if (!self.marcStyle & !self.tuseshadow)
      {
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        num =  Math.Round(( (self.width - 6) -  sizeF2.Width) / 2.0);
        y1 =  Math.Round(( (self.height + 2) -  sizeF2.Height) / 2.0 - 1.0);
        DrawMod.DrawTextVic3( Expression, self.buttext, self.ourfont, num, y1, DrawMod.TGame.VicColor2, DrawMod.TGame.VicColor1Shade);
      }
      else if (self.extraS.Length > 0)
      {
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x1: i32 =  Math.Round(2.0 + ( self.width -  sizeF2.Width) / 2.0);
        let mut y2: i32 =  Math.Round(2.0 + ( self.height -  sizeF2.Height) / 2.0 - 8.0);
        DrawMod.DrawTextColouredFuzzy( Expression, self.buttext, self.ourfont, x1, y2, Color.FromArgb(200, 0, 0, 0));
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x2: i32 =  Math.Round(( self.width -  sizeF2.Width) / 2.0);
        let mut y3: i32 =  Math.Round(( self.height -  sizeF2.Height) / 2.0 - 8.0);
        DrawMod.DrawTextColouredNicely( Expression, self.buttext, self.ourfont, x2, y3, Color.White);
        sizeF2 = Expression.MeasureString(self.extraS, self.ourfont2);
        let mut x3: i32 =  Math.Round(2.0 + ( self.width -  sizeF2.Width) / 2.0);
        let mut y4: i32 =  Math.Round(2.0 + ( self.height -  sizeF2.Height) / 2.0 + 8.0);
        DrawMod.DrawTextColouredFuzzy( Expression, self.extraS, self.ourfont2, x3, y4, Color.FromArgb(200, 0, 0, 0));
        sizeF2 = Expression.MeasureString(self.extraS, self.ourfont2);
        num =  Math.Round(( self.width -  sizeF2.Width) / 2.0);
        y1 =  Math.Round(( self.height -  sizeF2.Height) / 2.0 + 8.0);
        DrawMod.DrawTextColouredNicely( Expression, self.extraS, self.ourfont2, num, y1, Color.White);
      }
      else if (self.tuseshadow)
      {
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x: i32 =  Math.Round(0.0 + ( self.width -  sizeF2.Width) / 2.0);
        let mut y5: i32 =  Math.Round(2.0 + ( self.height -  sizeF2.Height) / 2.0);
        DrawMod.DrawTextColouredFuzzy( Expression, self.buttext, self.ourfont, x, y5, Color.FromArgb(200, 0, 0, 0));
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        num =  Math.Round(( self.width -  sizeF2.Width) / 2.0 - 2.0);
        y1 =  Math.Round(0.0 + ( self.height -  sizeF2.Height) / 2.0);
        DrawMod.DrawTextColouredNicely( Expression, self.buttext, self.ourfont, num, y1, c);
      }
      else
      {
        SizeF sizeF3 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x4: i32 =  Math.Round(( self.width -  sizeF3.Width) / 2.0);
        let mut y6: i32 =  Math.Round(( self.height -  sizeF3.Height) / 2.0 - 1.0);
        DrawMod.DrawTextColoured( Expression, self.buttext, self.ourfont, x4, y6, Color.Black);
        SizeF sizeF4 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x5: i32 =  Math.Round(( self.width -  sizeF4.Width) / 2.0);
        let mut y7: i32 =  Math.Round(( self.height -  sizeF4.Height) / 2.0 + 1.0);
        DrawMod.DrawTextColoured( Expression, self.buttext, self.ourfont, x5, y7, Color.Black);
        SizeF sizeF5 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x6: i32 =  Math.Round(( self.width -  sizeF5.Width) / 2.0 + 1.0);
        let mut y8: i32 =  Math.Round(( self.height -  sizeF5.Height) / 2.0);
        DrawMod.DrawTextColoured( Expression, self.buttext, self.ourfont, x6, y8, Color.Black);
        SizeF sizeF6 = Expression.MeasureString(self.buttext, self.ourfont);
        let mut x7: i32 =  Math.Round(( self.width -  sizeF6.Width) / 2.0 - 1.0);
        let mut y9: i32 =  Math.Round(( self.height -  sizeF6.Height) / 2.0);
        DrawMod.DrawTextColoured( Expression, self.buttext, self.ourfont, x7, y9, Color.Black);
        sizeF2 = Expression.MeasureString(self.buttext, self.ourfont);
        num =  Math.Round(( self.width -  sizeF2.Width) / 2.0);
        y1 =  Math.Round(( self.height -  sizeF2.Height) / 2.0);
        DrawMod.DrawTextColoured( Expression, self.buttext, self.ourfont, num, y1, c);
      }
      if (!self.marcStyle && self.red)
      {
        if (self.height == 26)
          DrawMod.drawLine( Expression, num,  Math.Round( y1 +  sizeF2.Height - 1.0),  Math.Round( ( num + sizeF2.Width)),  Math.Round( y1 +  sizeF2.Height - 1.0),  DrawMod.TGame.viccolor7.R,  DrawMod.TGame.viccolor7.G,  DrawMod.TGame.viccolor7.B,  DrawMod.TGame.viccolor7.A);
        else
          DrawMod.drawLine( Expression, num,  Math.Round( y1 +  sizeF2.Height + 2.0),  Math.Round( ( num + sizeF2.Width)),  Math.Round( y1 +  sizeF2.Height + 2.0),  DrawMod.TGame.viccolor7.R,  DrawMod.TGame.viccolor7.G,  DrawMod.TGame.viccolor7.B,  DrawMod.TGame.viccolor7.A);
      }
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return self.OwnBitmap;
    }
  }
}
