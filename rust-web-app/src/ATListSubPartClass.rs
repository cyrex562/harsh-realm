// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ATListSubPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;

namespace WindowsApplication1
{
  pub class ATListSubPartClass : SubPartClass
  {
     int ListSize;
     int ListSelect;
    pub TopItem: i32;
     ATListClass ListObj;
     OwnFont: Font;
     ownfont2: Font;
     const let mut ItemSize: i32 =  16;
     int ItemFontOffset;
     int LeftTextOffset;
     int Width;
     int Height;
     game: GameClass;
     Header: String;
     bool HeaderCenter;
     bool Highlight;
     bool ShowPair;
     int ValueWidth;
     bool DoTopAndBottom;
     backbitmap: Bitmap;
     int clickscroll;

    pub fn Refresh(ATListClass tListObj, int tlistselect, theader: String = "")
    {
      this.ListObj = tListObj;
      this.ListSelect = tlistselect;
      if (Strings.Len(theader) > 0)
        this.Header = theader;
      if (this.TopItem > this.ListObj.ListCount - this.ListSize)
        this.TopItem = this.ListObj.ListCount - this.ListSize;
      if (0 > this.TopItem)
        this.TopItem = 0;
      this.Clear();
    }

    pub fn SubDispose()
    {
      if (Information.IsNothing( this.backbitmap))
        return;
      this.backbitmap.Dispose();
      this.backbitmap = (Bitmap) null;
    }

    pub ATListSubPartClass(
      ATListClass tListobj,
      int tlistsize,
      int twidth,
      int tlistselect,
      tgame: GameClass,
      bool systemfont = false,
      tHeader: String = "",
      bool tHeaderCenter = true,
      bool tHighlight = true,
      let mut tTop: i32 =  0,
      bool tShowPair = false,
      let mut tValueWidth: i32 =  50,
      bool tdotopandbottom = true,
      ref tbackbitmap: Bitmap = null,
      let mut bbx: i32 =  -1,
      let mut bby: i32 =  -1)
      : base(twidth, (tlistsize + 3) * 16)
    {
      this.ItemFontOffset = 3;
      this.LeftTextOffset = 5;
      if (!tdotopandbottom)
        this.Resize(twidth, (tlistsize + 1) * 16);
      this.ShowPair = tShowPair;
      this.ValueWidth = tValueWidth;
      this.DoTopAndBottom = tdotopandbottom;
      if (!Information.IsNothing( tbackbitmap))
      {
        this.backbitmap = new Bitmap(this.OwnBitmap.Width, this.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
        this.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics graphics = Graphics.FromImage((Image) this.backbitmap);
        graphics.CompositingMode = CompositingMode.SourceCopy;
        graphics.DrawImage((Image) tbackbitmap, Rectangle::new(0, 0, this.OwnBitmap.Width, this.OwnBitmap.Height), Rectangle::new(bbx, bby, this.OwnBitmap.Width, this.OwnBitmap.Height), GraphicsUnit.Pixel);
        graphics.CompositingMode = CompositingMode.SourceOver;
      }
      this.Width = twidth;
      this.Height = (tlistsize + 3) * 16;
      if (!this.DoTopAndBottom)
        this.Height = (tlistsize + 1) * 16;
      this.ListSize = tlistsize;
      this.ListSelect = tlistselect;
      this.ListObj = tListobj;
      this.MouseOver = true;
      this.clickscroll = 0;
      this.Highlight = tHighlight;
      this.TopItem = tTop;
      this.HeaderCenter = tHeaderCenter;
      this.game = tgame;
      if (Strings.Len(tHeader) > 0)
        this.Header = tHeader;
      this.Header = Strings.UCase(this.Header);
      if (tTop == 0)
      {
        this.TopItem = (int) Math.Round( this.ListSelect - Conversion.Int( this.ListSize / 2.0));
        if (this.TopItem < 0)
          this.TopItem = 0;
      }
      if (!systemfont)
      {
        if (!this.DoTopAndBottom)
        {
          this.OwnFont = this.game.VicFont2;
          this.ownfont2 = this.game.VicFont2;
          this.ItemFontOffset = 1;
        }
        else
        {
          this.OwnFont = this.game.VicFont3;
          this.ownfont2 = this.game.VicFont3;
          this.ItemFontOffset = 1;
          this.LeftTextOffset = 2;
        }
      }
      else if (!this.DoTopAndBottom)
      {
        this.OwnFont = this.game.VicFont3;
        this.ownfont2 = this.game.VicFont3;
        this.ItemFontOffset = 1;
      }
      else
      {
        this.OwnFont = this.game.VicFont3;
        this.ownfont2 = this.game.VicFont3;
        this.ItemFontOffset = 0;
        this.LeftTextOffset = 2;
      }
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Color.FromArgb(0, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      Color.FromArgb(0, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (this.ListSize >= this.ListObj.ListCount)
        this.TopItem = 0;
      if (Information.IsNothing( this.backbitmap))
      {
        Expression.Clear(Color.Transparent);
      }
      else
      {
        Expression.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple(ref Expression, ref this.backbitmap, 0, 0);
        Expression.CompositingMode = CompositingMode.SourceOver;
      }
      if (this.DoTopAndBottom)
        DrawMod.DrawBlock(ref Expression, 0, 16, this.Width, this.Height - 32, (int) DrawMod.TGame.VicColor4.R, (int) DrawMod.TGame.VicColor4.G, (int) DrawMod.TGame.VicColor4.B, (int) DrawMod.TGame.VicColor4.A);
      else
        DrawMod.DrawBlock(ref Expression, 0, 0, this.Width, this.Height, (int) DrawMod.TGame.VicColor4.R, (int) DrawMod.TGame.VicColor4.G, (int) DrawMod.TGame.VicColor4.B, (int) DrawMod.TGame.VicColor4.A);
      let mut num1: i32 =  20;
      if (this.ListSize >= this.ListObj.ListCount)
        num1 = 0;
      let mut num2: i32 =  2;
      let mut num3: i32 =  1;
      if (!this.DoTopAndBottom)
      {
        num2 = 0;
        num3 = 0;
      }
      let mut num4: i32 =  -1;
      let mut topItem: i32 =  this.TopItem;
      let mut num5: i32 =  this.TopItem + this.ListSize + num2;
      bitmap: Bitmap;
      Rectangle rectangle1;
      Rectangle rectangle2;
      for (let mut index: i32 =  topItem; index <= num5; index += 1)
      {
        num4 += 1;
        if (!(num4 == this.ListSize + 2 & this.DoTopAndBottom))
        {
          if ((num4 + 10) % 2 == 0)
          {
            if (this.ListSelect == index - num3 & this.ListSelect > -1 & this.Highlight)
              DrawMod.DrawBlockGradient2(ref Expression, 0, 16 * num4, this.Width - num1, 16, Color.FromArgb(64, (int) this.game.viccolor7.R, (int) this.game.viccolor7.G, (int) this.game.viccolor7.B), this.game.viccolor7);
          }
          else if (this.ListSelect == index - num3 & this.ListSelect > -1 & this.Highlight)
            DrawMod.DrawBlockGradient2(ref Expression, 0, 16 * num4, this.Width - num1, 16, Color.FromArgb(64, (int) this.game.viccolor7.R, (int) this.game.viccolor7.G, (int) this.game.viccolor7.B), this.game.viccolor7);
        }
        if (num4 == 0 & this.DoTopAndBottom)
        {
          if (Strings.Len(this.Header) > 0)
          {
            SizeF sizeF2 = Expression.MeasureString(this.Header, this.ownfont2);
            if (this.HeaderCenter)
              DrawMod.DrawTextVic(ref Expression, this.Header, this.ownfont2, (int) Math.Round( this.Width / 2.0 -  sizeF2.Width / 2.0), 16 * num4 + this.ItemFontOffset - 1, this.game.VicColor1, this.game.VicColor1Shade);
            else
              DrawMod.DrawTextVic(ref Expression, this.Header, this.ownfont2, this.LeftTextOffset, 16 * num4 + this.ItemFontOffset - 1, this.game.VicColor1, this.game.VicColor1Shade);
          }
          if (this.TopItem > 0)
          {
            if (num1 > 0)
            {
              ref Graphics local1 = ref Expression;
              bitmap = BitmapStore.GetBitmap(this.game.LISTUP);
              ref local2: Bitmap = ref bitmap;
              let mut x: i32 =  this.Width - 20;
              DrawMod.DrawSimple(ref local1, ref local2, x, 3);
            }
          }
          else if (num1 > 0)
          {
            ref Graphics local3 = ref Expression;
            bitmap = BitmapStore.GetBitmap(this.game.LISTBLOCK);
            ref local4: Bitmap = ref bitmap;
            let mut x: i32 =  this.Width - 20;
            DrawMod.DrawSimple(ref local3, ref local4, x, 3);
          }
        }
        else if (num4 == this.ListSize + 2 & this.DoTopAndBottom)
        {
          if (this.TopItem + this.ListSize >= this.ListObj.ListCount)
          {
            if (num1 > 0)
            {
              ref Graphics local5 = ref Expression;
              bitmap = BitmapStore.GetBitmap(this.game.LISTBLOCK);
              ref local6: Bitmap = ref bitmap;
              let mut x: i32 =  this.Width - 20;
              let mut y: i32 =  16 * num4 + 3;
              DrawMod.DrawSimple(ref local5, ref local6, x, y);
            }
          }
          else if (num1 > 0)
          {
            ref Graphics local7 = ref Expression;
            bitmap = BitmapStore.GetBitmap(this.game.LISTDOWN);
            ref local8: Bitmap = ref bitmap;
            let mut x: i32 =  this.Width - 20;
            let mut y: i32 =  16 * num4 + 3;
            DrawMod.DrawSimple(ref local7, ref local8, x, y);
          }
        }
        else if (index - num3 <= this.ListObj.ListCount)
        {
          if (!this.ShowPair)
          {
            if (this.ListObj.ListColor[index - num3] < 0)
            {
              if (!Information.IsNothing( this.ListObj.ListBmp[index - num3]))
              {
                if (this.ListObj.ListR[index - num3] > -1)
                  DrawMod.DrawBlockGradient2(ref Expression, 0, 16 * num4, 24, 16, Color.FromArgb((int) byte.MaxValue, (int) Math.Round( this.ListObj.ListR[index - num3] / 2.0), (int) Math.Round( this.ListObj.ListG[index - num3] / 2.0), (int) Math.Round( this.ListObj.ListB[index - num3] / 2.0)), Color.FromArgb((int) byte.MaxValue, (int) Math.Round( this.ListObj.ListR[index - num3] / 4.0), (int) Math.Round( this.ListObj.ListG[index - num3] / 4.0), (int) Math.Round( this.ListObj.ListB[index - num3] / 4.0)));
                ref Graphics local9 = ref Expression;
                ref local10: Bitmap = ref this.ListObj.ListBmp[index - num3];
                rectangle1 = Rectangle::new(0, 0, 32, 16);
                let mut srcrect: &Rectangle = &rectangle1
                rectangle2 = Rectangle::new(this.LeftTextOffset, 16 * num4 + 3, 32, 16);
                let mut destrect: &Rectangle = &rectangle2
                DrawMod.DrawSimplePart2(ref local9, ref local10, srcrect, destrect);
                DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListName[index - num3], this.OwnFont, this.ListObj.ListBmp[index - num3].Width + 7 + this.LeftTextOffset, 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
              }
              else
                DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListName[index - num3], this.OwnFont, this.LeftTextOffset, 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
            }
            else
              DrawMod.DrawTextColoured(ref Expression, this.ListObj.ListName[index - num3], this.OwnFont, this.LeftTextOffset, 16 * num4 + this.ItemFontOffset, Color.FromArgb((int) byte.MaxValue, 0, 150, 0));
          }
          else
          {
            if (this.ListSize < this.ListObj.ListCount)
              DrawMod.drawLine(ref Expression, 0, 16 * num4, this.Width - 21, 16 * num4, (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
            else
              DrawMod.drawLine(ref Expression, 0, 16 * num4, this.Width - 1, 16 * num4, (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
            DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListName[index - num3], this.OwnFont, this.LeftTextOffset, 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
            if (Operators.CompareString(this.ListObj.ListValue4[index - num3], "", false) != 0)
            {
              DrawMod.drawLine(ref Expression, this.Width - num1 - (this.ValueWidth - 3), 16 * num4, this.Width - num1 - (this.ValueWidth - 3), 16 * (num4 + 1), (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
              DrawMod.drawLine(ref Expression, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth * 0.75 - 3.0)), 16 * num4, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth * 0.75 - 3.0)), 16 * (num4 + 1), (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
              DrawMod.drawLine(ref Expression, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth * 0.5 - 3.0)), 16 * num4, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth * 0.5 - 3.0)), 16 * (num4 + 1), (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
              DrawMod.drawLine(ref Expression, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth * 0.25 - 3.0)), 16 * num4, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth * 0.25 - 3.0)), 16 * (num4 + 1), (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
              DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListValue[index - num3], this.OwnFont, this.Width - num1 - (this.ValueWidth - 3), 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
              DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListValue2[index - num3], this.OwnFont, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth * 0.75 - 3.0)), 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
              DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListValue3[index - num3], this.OwnFont, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth * 0.5 - 3.0)), 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
              DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListValue4[index - num3], this.OwnFont, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth * 0.25 - 3.0)), 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
            }
            else if (Operators.CompareString(this.ListObj.ListValue3[index - num3], "", false) != 0)
            {
              DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListValue[index - num3], this.OwnFont, this.Width - num1 - (this.ValueWidth - 3), 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
              DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListValue2[index - num3], this.OwnFont, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth * 0.66 - 3.0)), 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
              DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListValue3[index - num3], this.OwnFont, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth * 0.33 - 3.0)), 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
            }
            else if (Operators.CompareString(this.ListObj.ListValue2[index - num3], "", false) != 0)
            {
              DrawMod.drawLine(ref Expression, this.Width - num1 - (this.ValueWidth - 3), 16 * num4, this.Width - num1 - (this.ValueWidth - 3), 16 * (num4 + 1), (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
              DrawMod.drawLine(ref Expression, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth / 2.0 - 3.0)), 16 * num4, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth / 2.0 - 3.0)), 16 * (num4 + 1), (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
              DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListValue[index - num3], this.OwnFont, this.Width - num1 - (this.ValueWidth - 3), 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
              DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListValue2[index - num3], this.OwnFont, (int) Math.Round( (this.Width - num1) - ( this.ValueWidth / 2.0 - 3.0)), 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
            }
            else
            {
              DrawMod.drawLine(ref Expression, this.Width - num1 - (this.ValueWidth - 3), 16 * num4, this.Width - num1 - (this.ValueWidth - 3), 16 * (num4 + 1), (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
              DrawMod.DrawTextVic2(ref Expression, this.ListObj.ListValue[index - num3], this.OwnFont, this.Width - num1 - (this.ValueWidth - 3), 16 * num4 + this.ItemFontOffset, this.game.VicColor2, this.game.VicColor2Shade);
            }
          }
        }
      }
      let mut num6: i32 =  (this.ListSize + 1) * 16;
      float num7 = this.ListObj.ListCount <= 0 ? 1f :  this.ListSize /  this.ListObj.ListCount;
      if ( num7 > 1.0)
        num7 = 1f;
      let mut num8: i32 =  (int) Math.Round( Conversion.Int( num6 * num7));
      float num9 = this.ListObj.ListCount <= 0 ? 0.0f :  this.TopItem /  this.ListObj.ListCount;
      if ( num9 > 1.0)
        num9 = 1f;
      let mut num10: i32 =  (int) Math.Round( Conversion.Int( num6 * num9));
      if (this.DoTopAndBottom)
        num10 += 16;
      if (num6 < 5)
        num6 = 5;
      if (num10 + num8 > num6 + 16)
        num8 = num6 + 16 - num10;
      if (this.ListSize < this.ListObj.ListCount)
      {
        let mut x: i32 =  this.Width - 19;
        let mut y: i32 =  num10 + 3;
        let mut width: i32 =  16;
        let mut num11: i32 =  num8 - 2;
        if (!this.DoTopAndBottom)
        {
          ref Graphics local11 = ref Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
          ref local12: Bitmap = ref bitmap;
          rectangle2 = Rectangle::new(0, 2, 20, 6);
          let mut srcrect1: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, 2, 20, this.Height - 4);
          let mut destrect1: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local11, ref local12, srcrect1, destrect1);
          ref Graphics local13 = ref Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
          ref local14: Bitmap = ref bitmap;
          rectangle2 = Rectangle::new(0, 0, 20, 2);
          let mut srcrect2: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, 0, 20, 2);
          let mut destrect2: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local13, ref local14, srcrect2, destrect2);
          ref Graphics local15 = ref Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
          ref local16: Bitmap = ref bitmap;
          rectangle2 = Rectangle::new(0, 8, 20, 2);
          let mut srcrect3: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, this.Height - 2, 20, 2);
          let mut destrect3: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local15, ref local16, srcrect3, destrect3);
        }
        else
        {
          ref Graphics local17 = ref Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
          ref local18: Bitmap = ref bitmap;
          rectangle2 = Rectangle::new(0, 2, 20, 6);
          let mut srcrect4: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, 18, 20, this.Height - 36);
          let mut destrect4: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local17, ref local18, srcrect4, destrect4);
          ref Graphics local19 = ref Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
          ref local20: Bitmap = ref bitmap;
          rectangle2 = Rectangle::new(0, 0, 20, 2);
          let mut srcrect5: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, 16, 20, 2);
          let mut destrect5: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local19, ref local20, srcrect5, destrect5);
          ref Graphics local21 = ref Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
          ref local22: Bitmap = ref bitmap;
          rectangle2 = Rectangle::new(0, 8, 20, 2);
          let mut srcrect6: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, this.Height - 18, 20, 2);
          let mut destrect6: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local21, ref local22, srcrect6, destrect6);
        }
        ref Graphics local23 = ref Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
        ref local24: Bitmap = ref bitmap;
        rectangle2 = Rectangle::new(0, 2, 20, 6);
        let mut srcrect7: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x + 1, y, width, num11 - 2);
        let mut destrect7: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2(ref local23, ref local24, srcrect7, destrect7);
        ref Graphics local25 = ref Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
        ref local26: Bitmap = ref bitmap;
        rectangle2 = Rectangle::new(0, 0, 20, 2);
        let mut srcrect8: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x + 1, y - 2, width, 2);
        let mut destrect8: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2(ref local25, ref local26, srcrect8, destrect8);
        ref Graphics local27 = ref Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
        ref local28: Bitmap = ref bitmap;
        rectangle2 = Rectangle::new(0, 8, 20, 2);
        let mut srcrect9: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x + 1, y + num11 - 2, 10, 2);
        let mut destrect9: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2(ref local27, ref local28, srcrect9, destrect9);
      }
      if (this.DoTopAndBottom)
      {
        if (this.ListSize < this.ListObj.ListCount)
          DrawMod.DrawRectangle(ref Expression, 0, 16, this.Width - 21, this.Height - 32, (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
        else
          DrawMod.DrawRectangle(ref Expression, 0, 16, this.Width - 1, this.Height - 32, (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
      }
      else if (this.ListSize < this.ListObj.ListCount)
        DrawMod.DrawRectangle(ref Expression, 0, 0, this.Width - 21, this.Height - 1, (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
      else
        DrawMod.DrawRectangle(ref Expression, 0, 0, this.Width - 1, this.Height - 1, (int) DrawMod.TGame.VicColor3.R, (int) DrawMod.TGame.VicColor3.G, (int) DrawMod.TGame.VicColor3.B, (int) DrawMod.TGame.VicColor3.A);
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return this.OwnBitmap;
    }

    pub int GetSelect() => this.ListSelect < 0 || this.ListSelect > this.ListObj.ListCount ? -1 : this.ListObj.ListData[this.ListSelect];

    pub int Click(int x, int y, let mut b: i32 =  1)
    {
      let mut num1: i32 =  y;
      y = (int) Math.Round(Conversion.Int( y / 16.0));
      this.Scroller = true;
      let mut num2: i32 =  0;
      let mut num3: i32 =  2;
      let mut num4: i32 =  1;
      if (!this.DoTopAndBottom)
      {
        num3 = 1;
        num2 = -1;
        num4 = 0;
      }
      let mut num5: i32 =  20;
      if (this.ListSize >= this.ListObj.ListCount)
        num5 = 0;
      if (y > num2 & y < this.ListSize + num3)
      {
        if (x <= this.Width - num5)
        {
          y -= num4;
          y += this.TopItem;
          this.clickscroll = 0;
          if (y > this.ListObj.ListCount)
            return -1;
          this.ListSelect = y;
          return this.ListObj.ListData[this.ListSelect];
        }
        this.clickscroll = 1;
        let mut num6: i32 =  (this.ListSize + 1) * 16;
        if (this.DoTopAndBottom)
          num1 -= 16;
        if (num1 < 1)
          num1 = 1;
        let mut num7: i32 =  (int) Math.Round( (int) Math.Round( ( num1 /  num6 *  this.ListObj.ListCount)) -  this.ListSize / 2.0);
        if (0 > num7)
          num7 = 0;
        this.TopItem = num7;
        if (this.TopItem > this.ListObj.ListCount - this.ListSize)
          this.TopItem = this.ListObj.ListCount - this.ListSize;
        if (0 > this.TopItem)
          this.TopItem = 0;
        return -1;
      }
      if (y == 0 & this.DoTopAndBottom)
      {
        --this.TopItem;
        this.clickscroll = 0;
        if (0 > this.TopItem)
          this.TopItem = 0;
        return -1;
      }
      if (!(y == this.ListSize + 2 & this.DoTopAndBottom))
        return -1;
      this += 1.TopItem;
      this.clickscroll = 0;
      if (this.TopItem > this.ListObj.ListCount - this.ListSize)
        this.TopItem = this.ListObj.ListCount - this.ListSize;
      if (0 > this.TopItem)
        this.TopItem = 0;
      return -1;
    }

    pub int HandleMouseUp(int x, int y)
    {
      if (!(this.clickscroll == 1 | this.Scroller))
        return -1;
      this.clickscroll = 0;
      this.Scroller = false;
      return 1;
    }

    pub bool MouseMove(int x, int y)
    {
      let mut num1: i32 =  y;
      y = (int) Math.Round(Conversion.Int( y / 16.0));
      let mut num2: i32 =  0;
      let mut num3: i32 =  2;
      let mut num4: i32 =  1;
      if (!this.DoTopAndBottom)
      {
        num3 = 1;
        num2 = -1;
        num4 = 0;
      }
      let mut num5: i32 =  20;
      if (this.ListSize >= this.ListObj.ListCount)
        num5 = 0;
      if (!(y > num2 & y < this.ListSize + num3 & this.clickscroll == 1))
        return false;
      let mut num6: i32 =  (this.ListSize + 1) * 16;
      if (this.DoTopAndBottom)
        num1 -= 16;
      if (num1 < 1)
        num1 = 1;
      let mut num7: i32 =  (int) Math.Round( (int) Math.Round( ( num1 /  num6 *  this.ListObj.ListCount)) -  this.ListSize / 2.0);
      if (0 > num7)
        num7 = 0;
      this.TopItem = num7;
      if (this.TopItem > this.ListObj.ListCount - this.ListSize)
        this.TopItem = this.ListObj.ListCount - this.ListSize;
      if (0 > this.TopItem)
        this.TopItem = 0;
      return true;
    }
  }
}
