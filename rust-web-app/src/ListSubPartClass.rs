// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ListSubPartClass
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
  pub class ListSubPartClass : SubPartClass
  {
     ListSize: i32;
     ListSelect: i32;
    pub TopItem: i32;
     ListClass ListObj;
     OwnFont: Font;
     ownfont2: Font;
     object ItemSize;
     ItemFontOffset: i32;
     LeftTextOffset: i32;
     Width: i32;
     Height: i32;
     game: GameClass;
     Header: String;
     bool HeaderCenter;
     bool Highlight;
     bool ShowPair;
     ValueWidth: i32;
     bool DoTopAndBottom;
     backbitmap: Bitmap;
     clickscroll: i32;
     bool MarcStyle;

    pub fn Refresh(ListClass tListObj, tlistselect: i32, theader: String = "")
    {
      self.ListObj = tListObj;
      self.ListSelect = tlistselect;
      if (Strings.Len(theader) > 0)
        self.Header = theader;
      if (self.TopItem > self.ListObj.ListCount - self.ListSize)
        self.TopItem = self.ListObj.ListCount - self.ListSize;
      if (0 > self.TopItem)
        self.TopItem = 0;
      self.Clear();
    }

    pub fn SubDispose()
    {
      if (Information.IsNothing( self.backbitmap))
        return;
      self.backbitmap.Dispose();
      self.backbitmap = (Bitmap) null;
    }

    pub ListSubPartClass(
      ListClass tListobj,
      tlistsize: i32,
      twidth: i32,
      tlistselect: i32,
      tgame: GameClass,
      bool systemfont = false,
      tHeader: String = "",
      bool tHeaderCenter = true,
      bool tHighlight = true,
      let mut tTop: i32 =  0,
      bool tShowPair = false,
      let mut tValueWidth: i32 =  50,
      bool tdotopandbottom = true,
       tbackbitmap: Bitmap = null,
      let mut bbx: i32 =  -1,
      let mut bby: i32 =  -1,
      bool tMarcStyle = false,
       overruleFont: Font = null,
      let mut overruleItemSize: i32 =  16)
      : base(twidth, (tlistsize + 3) * overruleItemSize)
    {
      self.ItemSize =  16;
      self.ItemFontOffset = 3;
      self.LeftTextOffset = 5;
      self.ItemSize =  overruleItemSize;
      if (!tdotopandbottom)
        self.Resize(twidth, Conversions.ToInteger(Operators.MultiplyObject( (tlistsize + 1), self.ItemSize)));
      self.MarcStyle = tMarcStyle;
      if (self.MarcStyle)
        self.LeftTextOffset = 10;
      self.ShowPair = tShowPair;
      self.ValueWidth = tValueWidth;
      self.DoTopAndBottom = tdotopandbottom;
      if (!Information.IsNothing( tbackbitmap))
      {
        self.backbitmap = new Bitmap(self.OwnBitmap.Width, self.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
        self.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics graphics = Graphics.FromImage((Image) self.backbitmap);
        graphics.CompositingMode = CompositingMode.SourceCopy;
        graphics.DrawImage((Image) tbackbitmap, Rectangle::new(0, 0, self.OwnBitmap.Width, self.OwnBitmap.Height), Rectangle::new(bbx, bby, self.OwnBitmap.Width, self.OwnBitmap.Height), GraphicsUnit.Pixel);
        graphics.CompositingMode = CompositingMode.SourceOver;
        graphics.Dispose();
      }
      self.Width = twidth;
      self.Height = Conversions.ToInteger(Operators.MultiplyObject( (tlistsize + 3), self.ItemSize));
      if (!self.DoTopAndBottom)
        self.Height = Conversions.ToInteger(Operators.MultiplyObject( (tlistsize + 1), self.ItemSize));
      self.ListSize = tlistsize;
      self.ListSelect = tlistselect;
      self.ListObj = tListobj;
      self.MouseOver = true;
      self.clickscroll = 0;
      self.Highlight = tHighlight;
      self.TopItem = tTop;
      self.HeaderCenter = tHeaderCenter;
      self.game = tgame;
      if (Strings.Len(tHeader) > 0)
        self.Header = tHeader;
      switch (tTop)
      {
        case -1:
          tTop = 0;
          self.TopItem = 0;
          break;
        case 0:
          self.TopItem =  Math.Round( self.ListSelect - Conversion.Int( self.ListSize / 2.0));
          if (self.TopItem < 0)
          {
            self.TopItem = 0;
            break;
          }
          break;
      }
      if (Information.IsNothing( overruleFont))
      {
        if (self.MarcStyle)
          self.OwnFont =  self.game.MarcFont5.Clone();
        else if (!systemfont)
        {
          self.OwnFont = Font::new(self.game.FontCol.Families[1], 12f, FontStyle.Regular, GraphicsUnit.Pixel);
          self.ownfont2 = Font::new(self.game.FontCol.Families[1], 11f, FontStyle.Regular, GraphicsUnit.Pixel);
        }
        else
        {
          self.OwnFont = Font::new("Courier New", 12f, FontStyle.Regular, GraphicsUnit.Pixel);
          self.ownfont2 = Font::new("Courier New", 11f, FontStyle.Regular, GraphicsUnit.Pixel);
        }
      }
      else
        self.OwnFont =  overruleFont.Clone();
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Color.FromArgb(0,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      color: Color = Color.FromArgb(0,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      if (self.ListSize >= self.ListObj.ListCount)
        self.TopItem = 0;
      if (Information.IsNothing( self.backbitmap))
      {
        Expression.Clear(Color.Transparent);
      }
      else
      {
        Expression.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( Expression,  self.backbitmap, 0, 0);
        Expression.CompositingMode = CompositingMode.SourceOver;
      }
      Rectangle rectangle1;
      Rectangle rectangle2;
      bitmap: Bitmap;
      if (self.MarcStyle)
      {
        if (self.MarcStyle)
          self.ItemFontOffset = 3;
        if (self.game.Data.Product == 7)
        {
          SizeF sizeF2 = Expression.MeasureString("Yj2", self.OwnFont);
          if (Operators.ConditionalCompareObjectLess( sizeF2.Height, Operators.SubtractObject(self.ItemSize,  4), false))
            self.ItemFontOffset += Conversions.ToInteger(NewLateBinding.LateGet( null, typeof (Math), "Floor", new object[1]
            {
              Operators.DivideObject(Operators.SubtractObject(Operators.SubtractObject(self.ItemSize,  4),  sizeF2.Height),  2)
            }, (string[]) null, (Type[]) null, (bool[]) null));
        }
        if (!self.DoTopAndBottom)
          DrawMod.DrawBlockGradient2( Expression, 0, 0, self.Width, self.Height, self.game.MarcCol1, self.game.MarcCol2);
        let mut num1: i32 =  20;
        if (self.ListSize >= self.ListObj.ListCount)
          num1 = 0;
        let mut num2: i32 =  2;
        let mut num3: i32 =  1;
        if (!self.DoTopAndBottom)
        {
          num2 = 0;
          num3 = 0;
        }
        let mut Right: i32 =  -1;
        let mut topItem: i32 =  self.TopItem;
        let mut num4: i32 =  self.TopItem + self.ListSize + num2;
        for (let mut index: i32 =  topItem; index <= num4; index += 1)
        {
          Right += 1;
          if (!(Right == self.ListSize + 2 & self.DoTopAndBottom) && self.ListSelect == index - num3 & self.ListSelect > -1 & self.Highlight)
          {
            if (self.MarcStyle)
              DrawMod.DrawBlockGradient2( Expression, 0, Conversions.ToInteger(Operators.MultiplyObject(self.ItemSize,  Right)), self.Width - num1, Conversions.ToInteger(self.ItemSize), Color.FromArgb(175,  byte.MaxValue, 200,  byte.MaxValue), Color.FromArgb(50,  byte.MaxValue, 200,  byte.MaxValue));
            else
              DrawMod.DrawBlockGradient2( Expression, 0, Conversions.ToInteger(Operators.MultiplyObject(self.ItemSize,  Right)), self.Width - num1, Conversions.ToInteger(self.ItemSize), Color.FromArgb(175, 0,  byte.MaxValue, 0), Color.FromArgb(50, 0,  byte.MaxValue, 0));
          }
          if (index - num3 <= self.ListObj.ListCount & index - num3 >= 0)
          {
            if (!self.ShowPair)
            {
              if (!Information.IsNothing( self.ListObj.ListBmp[index - num3]))
              {
                if (Operators.ConditionalCompareObjectLess( self.ListObj.ListBmp[index - num3].Height, self.ItemSize, false))
                {
                   let mut local1: &Graphics = &Expression;
                   local2: Bitmap =  self.ListObj.ListBmp[index - num3];
                  rectangle1 = Rectangle::new(0, 0, 32, self.ListObj.ListBmp[index - num3].Height);
                  let mut srcrect: &Rectangle = &rectangle1
                  rectangle2 = Rectangle::new(self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  1),  Conversions.ToInteger(Operators.DivideObject(Operators.SubtractObject(self.ItemSize,  self.ListObj.ListBmp[index - num3].Height),  2)))), 32, self.ListObj.ListBmp[index - num3].Height);
                  let mut destrect: &Rectangle = &rectangle2
                  DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
                }
                else
                {
                   let mut local3: &Graphics = &Expression;
                   local4: Bitmap =  self.ListObj.ListBmp[index - num3];
                  rectangle2 = Rectangle::new(0, 0, 32, Conversions.ToInteger(self.ItemSize));
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  3)), 32, Conversions.ToInteger(self.ItemSize));
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
                }
                if (self.ListObj.ListColor[index - num3] < 0)
                  DrawMod.DrawTextColouredMarc( Expression, self.ListObj.ListName[index - num3], self.OwnFont, self.ListObj.ListBmp[index - num3].Width + 7 + self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White);
                else
                  DrawMod.DrawTextColouredMarc( Expression, self.ListObj.ListName[index - num3], self.OwnFont, self.ListObj.ListBmp[index - num3].Width + 7 + self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.FromArgb( byte.MaxValue, 0, 150, 0));
              }
              else if (self.ListObj.ListColor[index - num3] < 0)
                DrawMod.DrawTextColouredMarc( Expression, self.ListObj.ListName[index - num3], self.OwnFont, self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White);
              else
                DrawMod.DrawTextColouredMarc( Expression, self.ListObj.ListName[index - num3], self.OwnFont, self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.FromArgb( byte.MaxValue, 0, 150, 0));
            }
            else
            {
              if (!Information.IsNothing( self.ListObj.ListBmp[index - num3]))
              {
                if (self.game.Data.Product >= 7)
                {
                  if (Operators.ConditionalCompareObjectLess( self.ListObj.ListBmp[index - num3].Height, self.ItemSize, false))
                  {
                     let mut local5: &Graphics = &Expression;
                     local6: Bitmap =  self.ListObj.ListBmp[index - num3];
                    rectangle2 = Rectangle::new(0, 0, self.ListObj.ListBmp[index - num3].Width, self.ListObj.ListBmp[index - num3].Height);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(self.LeftTextOffset, Conversions.ToInteger(Operators.MultiplyObject(self.ItemSize,  Right)), self.ListObj.ListBmp[index - num3].Width, Conversions.ToInteger(self.ItemSize));
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2( local5,  local6, srcrect, destrect);
                  }
                  else
                  {
                     let mut local7: &Graphics = &Expression;
                     local8: Bitmap =  self.ListObj.ListBmp[index - num3];
                    rectangle2 = Rectangle::new(0, 0, self.ListObj.ListBmp[index - num3].Width, self.ListObj.ListBmp[index - num3].Height);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  Conversions.ToInteger(Operators.DivideObject(Operators.SubtractObject( self.ListObj.ListBmp[index - num3].Height, self.ItemSize),  2)))), self.ListObj.ListBmp[index - num3].Width, self.ListObj.ListBmp[index - num3].Height);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2( local7,  local8, srcrect, destrect);
                  }
                  str: String = self.ListObj.ListName[index - num3];
                  if (Strings.InStr(str, "\r\n") > 1)
                  {
                    tstring1: String = Strings.Left(str, Strings.InStr(str, "\r\n") - 1);
                    tstring2: String = Strings.Mid(str, Strings.InStr(str, "\r\n") + 2);
                    DrawMod.DrawTextColouredMarc( Expression, tstring1, self.OwnFont, self.ListObj.ListBmp[index - num3].Width + 7 + self.LeftTextOffset, Conversions.ToInteger(Operators.SubtractObject(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset),   Math.Round( self.OwnFont.Height / 2.0))), Color.White);
                    DrawMod.DrawTextColouredMarc( Expression, tstring2, self.OwnFont, self.ListObj.ListBmp[index - num3].Width + 7 + self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset),   Math.Round( self.OwnFont.Height / 2.0))), Color.White);
                  }
                  else
                    DrawMod.DrawTextColouredMarc( Expression, self.ListObj.ListName[index - num3], self.OwnFont, self.ListObj.ListBmp[index - num3].Width + 7 + self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White);
                }
                else
                {
                   let mut local9: &Graphics = &Expression;
                   local10: Bitmap =  self.ListObj.ListBmp[index - num3];
                  rectangle2 = Rectangle::new(0, 0, 32, Conversions.ToInteger(self.ItemSize));
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  3)), 32, Conversions.ToInteger(self.ItemSize));
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2( local9,  local10, srcrect, destrect);
                  DrawMod.DrawTextColouredMarc( Expression, self.ListObj.ListName[index - num3], self.OwnFont, self.ListObj.ListBmp[index - num3].Width + 7 + self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White);
                }
              }
              else
                DrawMod.DrawTextColouredMarc( Expression, self.ListObj.ListName[index - num3], self.OwnFont, self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White);
              if (Operators.CompareString(self.ListObj.ListValue4[index - num3], "", false) != 0)
              {
                DrawMod.DrawTextColouredMarcCapped( Expression, self.ListObj.ListValue[index - num3], self.OwnFont, self.Width - num1 - (self.ValueWidth - 3), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White, maxWidth: ( Math.Round( self.ValueWidth * 0.25)));
                DrawMod.DrawTextColouredMarcCapped( Expression, self.ListObj.ListValue2[index - num3], self.OwnFont,  Math.Round( (self.Width - num1) - ( self.ValueWidth * 0.75 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White, maxWidth: ( Math.Round( self.ValueWidth * 0.25)));
                DrawMod.DrawTextColouredMarcCapped( Expression, self.ListObj.ListValue3[index - num3], self.OwnFont,  Math.Round( (self.Width - num1) - ( self.ValueWidth * 0.5 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White, maxWidth: ( Math.Round( self.ValueWidth * 0.25)));
                DrawMod.DrawTextColouredMarcCapped( Expression, self.ListObj.ListValue4[index - num3], self.OwnFont,  Math.Round( (self.Width - num1) - ( self.ValueWidth * 0.25 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White, maxWidth: ( Math.Round( self.ValueWidth * 0.25)));
              }
              else if (Operators.CompareString(self.ListObj.ListValue3[index - num3], "", false) != 0)
              {
                DrawMod.DrawTextColouredMarcCapped( Expression, self.ListObj.ListValue[index - num3], self.OwnFont, self.Width - num1 - (self.ValueWidth - 3), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White, maxWidth: ( Math.Round( self.ValueWidth * 0.33)));
                DrawMod.DrawTextColouredMarcCapped( Expression, self.ListObj.ListValue2[index - num3], self.OwnFont,  Math.Round( (self.Width - num1) - ( self.ValueWidth * 0.66 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White, maxWidth: ( Math.Round( self.ValueWidth * 0.33)));
                DrawMod.DrawTextColouredMarcCapped( Expression, self.ListObj.ListValue3[index - num3], self.OwnFont,  Math.Round( (self.Width - num1) - ( self.ValueWidth * 0.33 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White, maxWidth: ( Math.Round( self.ValueWidth * 0.33)));
              }
              else if (Operators.CompareString(self.ListObj.ListValue2[index - num3], "", false) != 0)
              {
                DrawMod.DrawTextColouredMarcCapped( Expression, self.ListObj.ListValue[index - num3], self.OwnFont, self.Width - num1 - (self.ValueWidth - 3), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White, maxWidth: ( Math.Round( self.ValueWidth * 0.5)));
                DrawMod.DrawTextColouredMarcCapped( Expression, self.ListObj.ListValue2[index - num3], self.OwnFont,  Math.Round( (self.Width - num1) - ( self.ValueWidth / 2.0 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White, maxWidth: ( Math.Round( self.ValueWidth * 0.5)));
              }
              else
                DrawMod.DrawTextColouredMarcCapped( Expression, self.ListObj.ListValue[index - num3], self.OwnFont, self.Width - num1 - (self.ValueWidth - 3), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.White, maxWidth: (self.ValueWidth * 1));
              if (index - num3 <= self.ListObj.ListCount & index > self.TopItem)
                DrawMod.drawLine( Expression, 0, Conversions.ToInteger(Operators.MultiplyObject(self.ItemSize,  Right)), self.Width, Conversions.ToInteger(Operators.MultiplyObject(self.ItemSize,  Right)),  self.game.MarcCol2.R,  self.game.MarcCol2.G,  self.game.MarcCol2.B,  self.game.MarcCol2.A);
              else
                index = index;
            }
          }
        }
      }
      else
      {
        if (self.DoTopAndBottom)
          DrawMod.DrawBlock( Expression, 0, Conversions.ToInteger(self.ItemSize), self.Width, Conversions.ToInteger(Operators.SubtractObject( self.Height, Operators.MultiplyObject( 2, self.ItemSize))), 0, 0, 0, 166);
        else
          DrawMod.DrawBlock( Expression, 0, 0, self.Width, self.Height, 0, 0, 0, 166);
        let mut num5: i32 =  20;
        if (self.ListSize >= self.ListObj.ListCount)
          num5 = 0;
        let mut num6: i32 =  2;
        let mut num7: i32 =  1;
        if (!self.DoTopAndBottom)
        {
          num6 = 0;
          num7 = 0;
        }
        let mut Right: i32 =  -1;
        let mut topItem: i32 =  self.TopItem;
        let mut num8: i32 =  self.TopItem + self.ListSize + num6;
        for (let mut index: i32 =  topItem; index <= num8; index += 1)
        {
          Right += 1;
          if (!(Right == self.ListSize + 2 & self.DoTopAndBottom))
          {
            if ((Right + 10) % 2 == 0)
            {
              if (self.ListSelect == index - num7 & self.ListSelect > -1 & self.Highlight)
                DrawMod.DrawBlockGradient2( Expression, 0, Conversions.ToInteger(Operators.MultiplyObject(self.ItemSize,  Right)), self.Width - num5, Conversions.ToInteger(self.ItemSize), Color.FromArgb(100, 100, 155, 100), Color.FromArgb(150, 100, 155, 100));
            }
            else if (self.ListSelect == index - num7 & self.ListSelect > -1 & self.Highlight)
              DrawMod.DrawBlockGradient2( Expression, 0, Conversions.ToInteger(Operators.MultiplyObject(self.ItemSize,  Right)), self.Width - num5, Conversions.ToInteger(self.ItemSize), Color.FromArgb(100, 100, 155, 100), Color.FromArgb(150, 100, 155, 100));
          }
          if (Right == 0 & self.DoTopAndBottom)
          {
            if (Strings.Len(self.Header) > 0)
            {
              SizeF sizeF3 = Expression.MeasureString(self.Header, self.ownfont2);
              if (self.HeaderCenter)
                DrawMod.DrawText( Expression, self.Header, self.ownfont2,  Math.Round( self.Width / 2.0 -  sizeF3.Width / 2.0), Conversions.ToInteger(Operators.SubtractObject(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset),  1)));
              else
                DrawMod.DrawText( Expression, self.Header, self.ownfont2, self.LeftTextOffset, Conversions.ToInteger(Operators.SubtractObject(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset),  1)));
            }
            if (self.TopItem > 0)
            {
              if (num5 > 0)
              {
                 let mut local11: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(self.game.LISTUP);
                 let mut local12: &Bitmap = &bitmap;
                let mut x: i32 =  self.Width - 20;
                DrawMod.DrawSimple( local11,  local12, x, 3);
              }
            }
            else if (num5 > 0)
            {
               let mut local13: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(self.game.LISTBLOCK);
               let mut local14: &Bitmap = &bitmap;
              let mut x: i32 =  self.Width - 20;
              DrawMod.DrawSimple( local13,  local14, x, 3);
            }
          }
          else if (Right == self.ListSize + 2 & self.DoTopAndBottom)
          {
            if (self.TopItem + self.ListSize >= self.ListObj.ListCount)
            {
              if (num5 > 0)
              {
                 let mut local15: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(self.game.LISTBLOCK);
                 let mut local16: &Bitmap = &bitmap;
                let mut x: i32 =  self.Width - 20;
                let mut integer: i32 =  Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  3));
                DrawMod.DrawSimple( local15,  local16, x, integer);
              }
            }
            else if (num5 > 0)
            {
               let mut local17: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(self.game.LISTDOWN);
               let mut local18: &Bitmap = &bitmap;
              let mut x: i32 =  self.Width - 20;
              let mut integer: i32 =  Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  3));
              DrawMod.DrawSimple( local17,  local18, x, integer);
            }
          }
          else if (index - num7 <= self.ListObj.ListCount)
          {
            if (!self.ShowPair)
            {
              if (self.ListObj.ListColor[index - num7] < 0)
                DrawMod.DrawText( Expression, self.ListObj.ListName[index - num7], self.OwnFont, self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
              else
                DrawMod.DrawTextColoured( Expression, self.ListObj.ListName[index - num7], self.OwnFont, self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)), Color.FromArgb( byte.MaxValue, 210, 170, 100));
            }
            else
            {
              if (self.ListSize < self.ListObj.ListCount)
                DrawMod.DrawRectangle( Expression, 0, Conversions.ToInteger(Operators.MultiplyObject(self.ItemSize,  Right)), self.Width - 21, Conversions.ToInteger(self.ItemSize),  color.R,  color.G,  color.B,  byte.MaxValue);
              else
                DrawMod.DrawRectangle( Expression, 0, Conversions.ToInteger(Operators.MultiplyObject(self.ItemSize,  Right)), self.Width - 1, Conversions.ToInteger(self.ItemSize),  color.R,  color.G,  color.B,  byte.MaxValue);
              DrawMod.DrawText( Expression, self.ListObj.ListName[index - num7], self.OwnFont, self.LeftTextOffset, Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
              if (Operators.CompareString(self.ListObj.ListValue4[index - num7], "", false) != 0)
              {
                DrawMod.DrawText( Expression, self.ListObj.ListValue[index - num7], self.OwnFont, self.Width - num5 - (self.ValueWidth - 3), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
                DrawMod.DrawText( Expression, self.ListObj.ListValue2[index - num7], self.OwnFont,  Math.Round( (self.Width - num5) - ( self.ValueWidth * 0.75 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
                DrawMod.DrawText( Expression, self.ListObj.ListValue3[index - num7], self.OwnFont,  Math.Round( (self.Width - num5) - ( self.ValueWidth * 0.5 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
                DrawMod.DrawText( Expression, self.ListObj.ListValue4[index - num7], self.OwnFont,  Math.Round( (self.Width - num5) - ( self.ValueWidth * 0.25 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
              }
              else if (Operators.CompareString(self.ListObj.ListValue3[index - num7], "", false) != 0)
              {
                DrawMod.DrawText( Expression, self.ListObj.ListValue[index - num7], self.OwnFont, self.Width - num5 - (self.ValueWidth - 3), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
                DrawMod.DrawText( Expression, self.ListObj.ListValue2[index - num7], self.OwnFont,  Math.Round( (self.Width - num5) - ( self.ValueWidth * 0.66 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
                DrawMod.DrawText( Expression, self.ListObj.ListValue3[index - num7], self.OwnFont,  Math.Round( (self.Width - num5) - ( self.ValueWidth * 0.33 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
              }
              else if (Operators.CompareString(self.ListObj.ListValue2[index - num7], "", false) != 0)
              {
                DrawMod.DrawText( Expression, self.ListObj.ListValue[index - num7], self.OwnFont, self.Width - num5 - (self.ValueWidth - 3), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
                DrawMod.DrawText( Expression, self.ListObj.ListValue2[index - num7], self.OwnFont,  Math.Round( (self.Width - num5) - ( self.ValueWidth / 2.0 - 3.0)), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
              }
              else
                DrawMod.DrawText( Expression, self.ListObj.ListValue[index - num7], self.OwnFont, self.Width - num5 - (self.ValueWidth - 3), Conversions.ToInteger(Operators.AddObject(Operators.MultiplyObject(self.ItemSize,  Right),  self.ItemFontOffset)));
            }
          }
        }
      }
      let mut Left: i32 =  Conversions.ToInteger(Operators.MultiplyObject( (self.ListSize + 1), self.ItemSize));
      float num9 = self.ListObj.ListCount <= 0 ? 1f :  self.ListSize /  self.ListObj.ListCount;
      if ( num9 > 1.0)
        num9 = 1f;
      let mut num10: i32 =   Math.Round( Conversion.Int( Left * num9));
      float num11 = self.ListObj.ListCount <= 0 ? 0.0f :  self.TopItem /  self.ListObj.ListCount;
      if ( num11 > 1.0)
        num11 = 1f;
      let mut num12: i32 =   Math.Round( Conversion.Int( Left * num11));
      if (self.DoTopAndBottom)
        num12 = Conversions.ToInteger(Operators.AddObject( num12, self.ItemSize));
      if (Left < 5)
        Left = 5;
      if (Operators.ConditionalCompareObjectGreater( (num12 + num10), Operators.AddObject( Left, self.ItemSize), false))
        num10 = Conversions.ToInteger(Operators.SubtractObject(Operators.AddObject( Left, self.ItemSize),  num12));
      if (self.ListSize < self.ListObj.ListCount)
      {
        let mut x: i32 =  self.Width - 18;
        let mut y: i32 =  num12 + 3;
        let mut width: i32 =  16;
        let mut num13: i32 =  num10 - 2;
        if (!self.MarcStyle)
        {
           let mut local19: &Graphics = &Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
           let mut local20: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 8, 28, 12);
          let mut srcrect1: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, y + 8, width, num13 - 16);
          let mut destrect1: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local19,  local20, srcrect1, destrect1);
           let mut local21: &Graphics = &Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
           let mut local22: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 0, 28, 8);
          let mut srcrect2: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, y, width, 8);
          let mut destrect2: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local21,  local22, srcrect2, destrect2);
           let mut local23: &Graphics = &Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
           let mut local24: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 28, 28, 8);
          let mut srcrect3: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, y + num13 - 8, 28, 8);
          let mut destrect3: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local23,  local24, srcrect3, destrect3);
        }
        else
        {
           let mut local25: &Graphics = &Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
           let mut local26: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 2, 20, 6);
          let mut srcrect4: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, 2, 20, self.Height - 4);
          let mut destrect4: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local25,  local26, srcrect4, destrect4);
           let mut local27: &Graphics = &Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
           let mut local28: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 0, 20, 2);
          let mut srcrect5: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, 0, 20, 2);
          let mut destrect5: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local27,  local28, srcrect5, destrect5);
           let mut local29: &Graphics = &Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
           let mut local30: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 8, 20, 2);
          let mut srcrect6: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, self.Height - 2, 20, 2);
          let mut destrect6: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local29,  local30, srcrect6, destrect6);
           let mut local31: &Graphics = &Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
           let mut local32: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 2, 20, 6);
          let mut srcrect7: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, y + 2, width, num13 - 2);
          let mut destrect7: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local31,  local32, srcrect7, destrect7);
           let mut local33: &Graphics = &Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
           let mut local34: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 0, 20, 2);
          let mut srcrect8: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, y, width, 2);
          let mut destrect8: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local33,  local34, srcrect8, destrect8);
           let mut local35: &Graphics = &Expression;
          bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
           let mut local36: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 8, 20, 2);
          let mut srcrect9: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, y + num13 - 2, 10, 2);
          let mut destrect9: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local35,  local36, srcrect9, destrect9);
        }
      }
      if (!self.MarcStyle)
      {
        if (self.DoTopAndBottom)
        {
          if (self.ListSize < self.ListObj.ListCount)
            DrawMod.DrawRectangle( Expression, 0, Conversions.ToInteger(self.ItemSize), self.Width - 21, Conversions.ToInteger(Operators.SubtractObject( self.Height, Operators.MultiplyObject(self.ItemSize,  2))),  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
          else
            DrawMod.DrawRectangle( Expression, 0, Conversions.ToInteger(self.ItemSize), self.Width - 1, Conversions.ToInteger(Operators.SubtractObject( self.Height, Operators.MultiplyObject(self.ItemSize,  2))),  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        }
        else if (self.ListSize < self.ListObj.ListCount)
          DrawMod.DrawRectangle( Expression, 0, 0, self.Width - 21, self.Height - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        else
          DrawMod.DrawRectangle( Expression, 0, 0, self.Width - 1, self.Height - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      }
      if (self.MarcStyle)
        DrawMod.DrawFrame( self.OwnBitmap,  self.backbitmap,  Expression, 0, 0, self.Width, self.Height, -1, -1);
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return self.OwnBitmap;
    }

    pub fn ShiftDown()
    {
      self += 1.ListSelect;
      if ( self.ListSelect >  self.TopItem +  self.ListSize / 2.0)
        self += 1.TopItem;
      if (self.TopItem > self.ListObj.ListCount - self.ListSize)
        self.TopItem = self.ListObj.ListCount - self.ListSize;
      if (self.ListSelect > self.ListObj.ListCount)
        self.ListSelect = self.ListObj.ListCount;
      if (0 > self.TopItem)
        self.TopItem = 0;
      if (0 <= self.ListSelect)
        return;
      self.ListSelect = 0;
    }

    pub fn ShiftUp()
    {
      --self.ListSelect;
      if ( (self.ListSelect - self.TopItem) <  self.ListSize / 2.0)
        --self.TopItem;
      if (self.TopItem > self.ListObj.ListCount - self.ListSize)
        self.TopItem = self.ListObj.ListCount - self.ListSize;
      if (self.ListSelect > self.ListObj.ListCount)
        self.ListSelect = self.ListObj.ListCount;
      if (0 > self.TopItem)
        self.TopItem = 0;
      if (0 <= self.ListSelect)
        return;
      self.ListSelect = 0;
    }

    pub fn GetSelect() -> i32 => self.ListObj.ListData[self.ListSelect];

    pub fn GetTopItem() -> i32 => self.TopItem;

    pub fn Click(x: i32, y: i32, let mut b: i32 =  1) -> i32
    {
      let mut Left: i32 =  y;
      y = Conversions.ToInteger(Conversion.Int(Operators.DivideObject( y, self.ItemSize)));
      self.Scroller = true;
      let mut num1: i32 =  0;
      let mut num2: i32 =  2;
      let mut num3: i32 =  1;
      if (!self.DoTopAndBottom)
      {
        num2 = 1;
        num1 = -1;
        num3 = 0;
      }
      let mut num4: i32 =  20;
      if (self.ListSize >= self.ListObj.ListCount)
        num4 = 0;
      if (y > num1 & y < self.ListSize + num2)
      {
        if (x <= self.Width - num4)
        {
          y -= num3;
          y += self.TopItem;
          self.clickscroll = 0;
          if (y > self.ListObj.ListCount)
            return -1;
          self.ListSelect = y;
          return self.ListObj.ListData[self.ListSelect];
        }
        self.clickscroll = 1;
        let mut integer: i32 =  Conversions.ToInteger(Operators.MultiplyObject( (self.ListSize + 1), self.ItemSize));
        if (self.DoTopAndBottom)
          Left = Conversions.ToInteger(Operators.SubtractObject( Left, self.ItemSize));
        if (Left < 1)
          Left = 1;
        let mut num5: i32 =   Math.Round(  Math.Round( ( Left /  integer *  self.ListObj.ListCount)) -  self.ListSize / 2.0);
        if (0 > num5)
          num5 = 0;
        self.TopItem = num5;
        if (self.TopItem > self.ListObj.ListCount - self.ListSize)
          self.TopItem = self.ListObj.ListCount - self.ListSize;
        if (0 > self.TopItem)
          self.TopItem = 0;
        return -1;
      }
      if (y == 0 & self.DoTopAndBottom)
      {
        --self.TopItem;
        self.clickscroll = 0;
        if (0 > self.TopItem)
          self.TopItem = 0;
        return -1;
      }
      if (!(y == self.ListSize + 2 & self.DoTopAndBottom))
        return -1;
      self += 1.TopItem;
      self.clickscroll = 0;
      if (self.TopItem > self.ListObj.ListCount - self.ListSize)
        self.TopItem = self.ListObj.ListCount - self.ListSize;
      if (0 > self.TopItem)
        self.TopItem = 0;
      return -1;
    }

    pub fn HandleMouseUp(x: i32, y: i32) -> i32
    {
      if (!(self.clickscroll == 1 | self.Scroller))
        return -1;
      self.clickscroll = 0;
      self.Scroller = false;
      return 1;
    }

    pub fn HandleBLOCKEDMouseUp(x: i32, y: i32) -> i32
    {
      if (!(self.clickscroll == 1 | self.Scroller))
        return -1;
      self.clickscroll = 0;
      self.Scroller = false;
      return 1;
    }

    pub bool MouseMove(x: i32, y: i32)
    {
      let mut Left: i32 =  y;
      y = Conversions.ToInteger(Conversion.Int(Operators.DivideObject( y, self.ItemSize)));
      let mut num1: i32 =  0;
      let mut num2: i32 =  2;
      let mut num3: i32 =  1;
      if (!self.DoTopAndBottom)
      {
        num2 = 1;
        num1 = -1;
        num3 = 0;
      }
      let mut num4: i32 =  20;
      if (self.ListSize >= self.ListObj.ListCount)
        num4 = 0;
      if (!(y > num1 & y < self.ListSize + num2 & self.clickscroll == 1))
        return false;
      let mut integer: i32 =  Conversions.ToInteger(Operators.MultiplyObject( (self.ListSize + 1), self.ItemSize));
      if (self.DoTopAndBottom)
        Left = Conversions.ToInteger(Operators.SubtractObject( Left, self.ItemSize));
      if (Left < 1)
        Left = 1;
      let mut num5: i32 =   Math.Round(  Math.Round( ( Left /  integer *  self.ListObj.ListCount)) -  self.ListSize / 2.0);
      if (0 > num5)
        num5 = 0;
      self.TopItem = num5;
      if (self.TopItem > self.ListObj.ListCount - self.ListSize)
        self.TopItem = self.ListObj.ListCount - self.ListSize;
      if (0 > self.TopItem)
        self.TopItem = 0;
      return true;
    }
  }
}
