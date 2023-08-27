// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TextAreaClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;

namespace WindowsApplication1
{
  pub class TextAreaClass : SubPartClass
  {
     ListSize: i32;
     ListSelect: i32;
    pub TopItem: i32;
     ListClass[] ListObj;
     Tab: i32;
     TabName: Vec<String>;
     TabCount: i32;
     OwnFont: Font;
     ItemSize: i32;
     const let mut ItemFontOffset: i32 = 1;
     const let mut LeftTextOffset: i32 = 15;
     Width: i32;
     Height: i32;
     Header: String;
     bool HeaderCenter;
     game: GameClass;
     backbitmap: Bitmap;
     bx: i32;
     by: i32;
     fontcol: Color;
     fontcol2: Color;
     bool centerit;
     clickscroll: i32;
     bool HideShade;
     Style: i32;
     scrollw: i32;
    pub BlockScroller: bool;
    pub MarcStyle: bool;
    pub Rectangle[] TabRect;

    pub fn SubDispose()
    {
      if (Information.IsNothing( self.backbitmap))
        return;
      self.backbitmap.Dispose();
      self.backbitmap = (Bitmap) null;
    }

    pub TextAreaClass(
      tgame: GameClass,
      twidth: i32,
      trows: i32,
      tfont: Font,
      theader: String,
      bool theadercenter,
      tText: String,
      tfontcol: Color,
      let mut tTop: i32 = 0,
      let mut tItemSize: i32 = 16,
       tbackbitmap: Bitmap = null,
      let mut bbx: i32 = -1,
      let mut bby: i32 = -1,
      bool tcenterit = false,
      bool tHideShade = false,
      let mut tStyle: i32 = 0,
      bool tBlockScroller = false)
      : base(twidth, (trows + 2) * tItemSize)
    {
      self.ListObj = new ListClass[10];
      self.TabName = new string[10];
      self.TabRect = Rectangle::new[100];
      self.ItemSize = tItemSize;
      self.Width = twidth;
      self.MarcStyle = false;
      self.Height = (trows + 1) * self.ItemSize;
      self.game = tgame;
      self.scrollw = 60;
      if (self.MarcStyle)
        self.scrollw = 45;
      self.BlockScroller = tBlockScroller;
      self.HideShade = tHideShade;
      self.centerit = tcenterit;
      if (!Information.IsNothing( tbackbitmap))
      {
        self.backbitmap = new Bitmap(self.OwnBitmap.Width, self.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
        self.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics graphics = Graphics.FromImage((Image) self.backbitmap);
        graphics.CompositingMode = CompositingMode.SourceCopy;
        graphics.DrawImage((Image) tbackbitmap, Rectangle::new(0, 0, self.OwnBitmap.Width, self.OwnBitmap.Height), Rectangle::new(bbx, bby, self.OwnBitmap.Width, self.OwnBitmap.Height), GraphicsUnit.Pixel);
        graphics.CompositingMode = CompositingMode.SourceOver;
      }
      if (Information.IsNothing( tfontcol))
      {
        self.fontcol = self.game.VicColor2;
        self.fontcol2 = self.game.VicColor2Shade;
      }
      else
      {
        self.fontcol = tfontcol;
        self.fontcol2 = Color.Transparent;
      }
      self.bx = bbx;
      self.by = bby;
      self.Style = tStyle;
      if (self.Style == 1 | self.Style == 2)
        self.scrollw = 35;
      SizeF sizeF = SizeF::new();
      strArray: Vec<String> = new string[10];
      self.Tab = 0;
      self.TabCount = -1;
      self.TabName[0] = "";
      let mut num1: i32 = 1;
      if (Information.IsNothing( tText))
        tText = "";
      tText = tText.Replace("\t", " ");
      while (num1 == 1)
      {
        num1 = 0;
        let mut num2: i32 = Strings.InStr(tText, "[tab]");
        if (num2 > 0)
        {
          let mut num3: i32 = Strings.InStr(tText, "[/tab]");
          if (num3 > num2 & num3 > 0)
          {
            str1: String = Strings.Mid(tText, num2 + Strings.Len("[tab]"), num3 - (num2 + Strings.Len("[tab]")));
            let mut num4: i32 = Strings.InStr(str1, ",");
            if (num4 > 0)
            {
              self += 1.TabCount;
              self.TabName[self.TabCount] = Strings.Left(str1, num4 - 1);
              str2: String = Strings.Mid(str1, num4 + 1);
              strArray[self.TabCount] = str2;
              tText = Strings.Left(tText, num2 - 1) + Strings.Mid(tText, num3 + Strings.Len("[/tab]"));
              num1 = 1;
            }
          }
        }
      }
      if (self.TabCount == -1)
      {
        self.TabCount = 0;
        strArray[0] = tText;
      }
      let mut tabCount: i32 = self.TabCount;
      for (let mut index: i32 = 0; index <= tabCount; index += 1)
      {
        self.ListObj[index] = ListClass::new();
        Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
        self.OwnFont = tfont != null ? tfont : tgame.VicFont2;
        let mut num5: i32 = 1;
        self.clickscroll = 0;
        tText = strArray[index];
        while (Strings.Len(tText) > 0)
        {
          let mut num6: i32 = 1;
          tname: String = "";
          while (num6 == 1)
          {
            let mut num7: i32 = Strings.InStr(tText, "\r\n");
            let mut num8: i32 = Strings.InStr(tText, " ");
            if (num8 == 0)
              num8 = 9999999;
            bool flag = false;
            if (num7 < num8 & num7 > 0)
            {
              flag = true;
              let mut num9: i32 = num7;
              num6 = 0;
              let mut num10: i32 = 0;
              if (num9 != 1)
              {
                if ( Expression.MeasureString(tname + Strings.Left(tText, num9 - 1), self.OwnFont).Width <=  (self.Width - self.scrollw))
                {
                  tname += Strings.Left(tText, num9 - 1);
                }
                else
                {
                  num10 = 1;
                  flag = false;
                }
              }
              if (num10 == 0)
                tText = num9 >= Strings.Len(tText) ? "" : Strings.Mid(tText, num9 + 2);
            }
            if (!flag)
            {
              let mut Length: i32 = Strings.InStr(tText, " ");
              str: String = Length <= 0 ? tText : Strings.Left(tText, Length);
              let mut num11: i32 = 0;
              num6 = 0;
              if ( Expression.MeasureString(tname + str, self.OwnFont).Width <=  (self.Width - self.scrollw))
              {
                num5 = 1;
                num11 = 1;
              }
              else if (num5 == 1)
              {
                num5 = 0;
              }
              else
              {
                num5 = 1;
                num11 = 1;
              }
              if (num11 == 1)
              {
                tname += str;
                if (Length > 0)
                {
                  if (Strings.Len(tText) >= Length + 1)
                  {
                    tText = Strings.Mid(tText, Length + 1);
                    num6 = 1;
                  }
                  else
                  {
                    tText = "";
                    num6 = 0;
                  }
                }
                else
                {
                  tText = "";
                  num6 = 0;
                }
              }
            }
          }
          self.ListObj[index].add(tname, 0);
        }
        if (!Information.IsNothing( Expression))
          Expression.Dispose();
      }
      self.ListSize = trows;
      if (self.TabCount > 0)
        --self.ListSize;
      self.ListSelect = -1;
      self.TopItem = tTop;
      self.HeaderCenter = theadercenter;
      if (Strings.Len(theader) > 0)
        self.Header = theader;
      if (tTop == 0)
      {
        self.TopItem =  Math.Round( self.ListSelect - Conversion.Int( self.ListSize / 2.0));
        if (self.TopItem < 0)
          self.TopItem = 0;
      }
      self.MouseOver = true;
      self.scrollw -= 20;
    }

    pub fn HeightUsed() => Math.Min(self.ListSize + 1, self.ListObj[self.Tab].ListCount + 1) -> i32 * self.ItemSize;

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
      if (!self.MarcStyle)
      {
        if (self.Style == 1)
          Expression.Clear(Color.Black);
        if (self.Style == 2)
          Expression.Clear(Color.White);
      }
      else if (self.TabCount > 0)
        DrawMod.DrawBlockGradient2( Expression, 0, self.ItemSize, self.Width, self.Height - self.ItemSize, self.game.MarcCol1, self.game.MarcCol2);
      else
        DrawMod.DrawBlockGradient2( Expression, 0, self.ItemSize, self.Width, self.Height, self.game.MarcCol1, self.game.MarcCol2);
      if (!self.MarcStyle)
      {
        num1: i32;
        if (self.TabCount == 0)
        {
          if (self.Style == 0 & !self.HideShade)
            DrawMod.DrawBlock( Expression, 0, 0, self.Width - (self.scrollw - 5), self.ItemSize * (self.ListSize + 1) + 2,  self.game.VicColor4.R,  self.game.VicColor4.G,  self.game.VicColor4.B,  self.game.VicColor4.A);
          num1 = -1;
        }
        else
        {
          if (self.Style == 0 & !self.HideShade)
            DrawMod.DrawBlock( Expression, 0, self.ItemSize, self.Width - (self.scrollw - 5), self.ItemSize * (self.ListSize + 0) + 2,  self.game.VicColor4.R,  self.game.VicColor4.G,  self.game.VicColor4.B,  self.game.VicColor4.A);
          num1 = 0;
          let mut w: i32 = Math.Min(120,  Math.Round( self.Width * 0.9 /  (self.TabCount + 1)));
          let mut x1: i32 = 0;
          let mut tabCount1: i32 = self.TabCount;
          for (let mut index: i32 = 0; index <= tabCount1; index += 1)
          {
            if (self.Tab != index)
            {
              DrawMod.DrawBlock( Expression, x1, 0, w, self.ItemSize,  self.game.VicColor4.R,  self.game.VicColor4.G,  self.game.VicColor4.B,  self.game.VicColor4.A);
              DrawMod.DrawTextVic2( Expression, self.TabName[index], Font::new(self.OwnFont.Name, self.OwnFont.Size - 2f, FontStyle.Regular, GraphicsUnit.Pixel), x1 + 1, 2, self.fontcol, self.fontcol2);
              DrawMod.DrawRectangle( Expression, x1, 0, w, self.ItemSize,  self.game.VicColor3.R,  self.game.VicColor3.G,  self.game.VicColor3.B,  self.game.VicColor3.A);
            }
            x1 += w;
          }
          let mut num2: i32 = 0;
          let mut tabCount2: i32 = self.TabCount;
          for (let mut index: i32 = 0; index <= tabCount2; index += 1)
          {
            if (index == self.Tab)
            {
              DrawMod.DrawBlock( Expression, num2 + 1, 0, w - 2, self.ItemSize,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
              DrawMod.DrawTextVic2( Expression, self.TabName[index], Font::new(self.OwnFont.Name, self.OwnFont.Size - 2f, FontStyle.Regular, GraphicsUnit.Pixel), num2 + 1, 2, Color.Black, Color.Transparent);
              DrawMod.DrawRectangle( Expression, num2 + 1, 0, w - 2, self.ItemSize - 1,  self.game.VicColor3.R,  self.game.VicColor3.G,  self.game.VicColor3.B,  self.game.VicColor3.A);
            }
            num2 += w;
          }
        }
        let mut num3: i32 = 0;
        if (self.TabCount > 0)
          num3 = -1;
        let mut topItem: i32 = self.TopItem;
        let mut num4: i32 = self.TopItem + self.ListSize + num3;
        for (let mut index: i32 = topItem; index <= num4; index += 1)
        {
          num1 += 1;
          if (index <= self.ListObj[self.Tab].ListCount)
          {
            str: String = self.ListObj[self.Tab].ListName[index];
            if (self.BlockScroller && index == self.TopItem + self.ListSize + num3 & index != self.ListObj[self.Tab].ListCount && Strings.Len(str) > 0)
              str = Strings.Left(str, Strings.Len(str) - 3) + "...";
            if (!self.centerit)
            {
              if (self.Style == 0)
                DrawMod.DrawTextVic2( Expression, str, self.OwnFont, 15, self.ItemSize * num1 + 1, self.fontcol, self.fontcol2);
              else
                DrawMod.DrawTextVic2( Expression, str, self.OwnFont, 15, self.ItemSize * num1 + 1, self.fontcol, self.fontcol2);
            }
            else
            {
              let mut num5: i32 =  Math.Round( (self.Width - self.scrollw) / 2.0 -  Expression.MeasureString(self.ListObj[self.Tab].ListName[index], self.OwnFont).Width / 2.0);
              if (0 > num5)
                num5 = 0;
              if (self.Style == 0)
                DrawMod.DrawTextVic2( Expression, str, self.OwnFont, num5 + 15, self.ItemSize * num1 + 1, self.fontcol, self.fontcol2);
              else
                DrawMod.DrawTextVic2( Expression, str, self.OwnFont, num5 + 15, self.ItemSize * num1 + 1, self.fontcol, self.fontcol2);
            }
          }
        }
      }
      else
      {
        num6: i32;
        if (self.TabCount == 0)
        {
          SizeF sizeF2 = Expression.MeasureString(Strings.UCase(self.Header), DrawMod.TGame.MarcFont5);
          DrawMod.DrawTextColouredMarc( Expression, Strings.UCase(self.Header), DrawMod.TGame.MarcFont5,  Math.Round(( self.Width -  sizeF2.Width) / 2.0), 0, Color.White);
        }
        else
        {
          num6 = 0;
          let mut num7: i32 = 0;
          let mut tabCount3: i32 = self.TabCount;
          for (let mut index: i32 = 0; index <= tabCount3; index += 1)
          {
            self.TabName[index] = Strings.UCase(self.TabName[index]);
            SizeF sizeF3 = index >= self.TabCount ? Expression.MeasureString(self.TabName[index], DrawMod.TGame.MarcFont5) : Expression.MeasureString(self.TabName[index] + " | ", DrawMod.TGame.MarcFont5);
            num7 =  Math.Round( ( num7 + sizeF3.Width));
          }
          let mut x: i32 =  Math.Round(Math.Max(0.0,  (self.Width - num7) / 2.0));
          let mut tabCount4: i32 = self.TabCount;
          for (let mut index: i32 = 0; index <= tabCount4; index += 1)
          {
            SizeF sizeF4;
            if (index < self.TabCount)
            {
              if (self.Tab != index)
                DrawMod.DrawTextColouredMarc( Expression, self.TabName[index] + " | ", DrawMod.TGame.MarcFont5, x + 1, 0, Color.White);
              if (index == self.Tab)
              {
                DrawMod.DrawTextColouredMarc( Expression, self.TabName[index], DrawMod.TGame.MarcFont5, x + 1, 0, DrawMod.TGame.MarcCol5);
                SizeF sizeF5 = Expression.MeasureString(self.TabName[index], DrawMod.TGame.MarcFont5);
                DrawMod.DrawTextColouredMarc( Expression, " | ", DrawMod.TGame.MarcFont5,  Math.Round( (x + 1) +  sizeF5.Width - 4.0), 0, Color.White);
              }
              sizeF4 = Expression.MeasureString(self.TabName[index] + " | ", DrawMod.TGame.MarcFont5);
            }
            else
            {
              if (self.Tab != index)
                DrawMod.DrawTextColouredMarc( Expression, self.TabName[index], DrawMod.TGame.MarcFont5, x + 1, 0, Color.White);
              if (index == self.Tab)
                DrawMod.DrawTextColouredMarc( Expression, self.TabName[index], DrawMod.TGame.MarcFont5, x + 1, 0, DrawMod.TGame.MarcCol5);
              sizeF4 = Expression.MeasureString(self.TabName[index], DrawMod.TGame.MarcFont5);
            }
            self.TabRect[index] = index >= self.TabCount ? Rectangle::new(x, 0,  Math.Round( sizeF4.Width), self.ItemSize) : Rectangle::new(x, 0,  Math.Round( sizeF4.Width -  Expression.MeasureString(" | ", DrawMod.TGame.MarcFont5).Width), self.ItemSize);
            x =  Math.Round( ( x + sizeF4.Width));
          }
        }
        let mut num8: i32 = -1;
        if (self.TabCount > 0)
          num8 = -2;
        let mut topItem: i32 = self.TopItem;
        let mut num9: i32 = self.TopItem + self.ListSize + num8;
        for (let mut index: i32 = topItem; index <= num9; index += 1)
        {
          num6 += 1;
          if (index <= self.ListObj[self.Tab].ListCount)
          {
            str: String = self.ListObj[self.Tab].ListName[index];
            if (self.BlockScroller && index == self.TopItem + self.ListSize + num8 & index != self.ListObj[self.Tab].ListCount)
              str = Strings.Left(str, Strings.Len(str) - 3) + "...";
            if (!self.centerit)
            {
              DrawMod.DrawTextColouredMarc( Expression, str, self.OwnFont, 15, self.ItemSize * num6 + 15, self.fontcol);
            }
            else
            {
              let mut num10: i32 =  Math.Round( (self.Width - self.scrollw) / 2.0 -  Expression.MeasureString(self.ListObj[self.Tab].ListName[index], self.OwnFont).Width / 2.0);
              if (0 > num10)
                num10 = 0;
              DrawMod.DrawTextColouredMarc( Expression, str, self.OwnFont, num10 + 15, self.ItemSize * num6 + 15, self.fontcol);
            }
          }
        }
      }
      if (!self.MarcStyle)
      {
        if (self.TabCount == 0)
        {
          if (self.Style == 0 & !self.HideShade)
            DrawMod.DrawRectangle( Expression, 0, 0, self.Width - (self.scrollw - 5), self.ItemSize * (self.ListSize + 1) + 2,  self.game.VicColor3.R,  self.game.VicColor3.G,  self.game.VicColor3.B,  self.game.VicColor3.A);
        }
        else if (self.Style == 0 & !self.HideShade)
          DrawMod.DrawRectangle( Expression, 0, self.ItemSize, self.Width - (self.scrollw - 5), self.ItemSize * (self.ListSize + 0) + 2,  self.game.VicColor3.R,  self.game.VicColor3.G,  self.game.VicColor3.B,  self.game.VicColor3.A);
      }
      if (self.ListSize < self.ListObj[self.Tab].ListCount & self.Style == 0 & !self.BlockScroller)
      {
        num11: i32;
        if (self.TabCount > 0)
        {
          if (!self.MarcStyle)
            DrawMod.DrawSteveBlock( Expression, self.Width - 29, self.ItemSize * 1, 28, self.ItemSize * (self.ListSize + 0));
          num11 = (self.ListSize + 0) * self.ItemSize;
        }
        else
        {
          if (!self.MarcStyle)
            DrawMod.DrawSteveBlock( Expression, self.Width - 29, self.ItemSize * 0, 28, self.ItemSize * (self.ListSize + 1));
          num11 = (self.ListSize + 1) * self.ItemSize;
        }
        float num12 = self.ListObj[self.Tab].ListCount <= 0 ? 1f : (self.TabCount <= 0 ?  self.ListSize /  self.ListObj[self.Tab].ListCount :  (self.ListSize - 1) /  self.ListObj[self.Tab].ListCount);
        if ( num12 > 1.0)
          num12 = 1f;
        let mut num13: i32 =  Math.Round( Conversion.Int( num11 * num12));
        float num14 = self.ListObj[self.Tab].ListCount <= 0 ? 0.0f :  self.TopItem /  self.ListObj[self.Tab].ListCount;
        if ( num14 > 1.0)
          num14 = 1f;
        if (self.ListSize < self.ListObj[self.Tab].ListCount)
        {
          let mut num15: i32 = !self.MarcStyle ?  Math.Round( Conversion.Int( num11 * num14)) :  Math.Round( Conversion.Int( num11 * num14));
          if (num13 < 20)
            num13 = 20;
          if (num15 + 2 + num13 > self.ItemSize * (self.ListSize + 1))
          {
            num15 -= self.ItemSize * (self.ListSize + 1) - (num15 + 2 + num13);
            if (0 > num15)
              num15 = 0;
          }
          let mut x: i32 = 1 + (self.Width - 24);
          let mut y: i32 = num15 + 2;
          let mut width: i32 = 20;
          let mut num16: i32 = num13 + self.ItemSize;
          if (self.TabCount > 0)
            y += self.ItemSize;
          if (self.TabCount == 0)
          {
            y += self.ItemSize;
            num16 += self.ItemSize;
          }
          if (self.TabCount > 0 && y + num16 > self.Height)
            num16 -= y + num16 - self.Height;
          if (!self.MarcStyle)
          {
            num17: i32;
            if (self.TabCount == 0)
            {
              y -= self.ItemSize;
              num17 = num16 - self.ItemSize * 2;
            }
            else
              num17 = num16 - self.ItemSize;
             let mut local1: &Graphics = &Expression;
            bitmap1: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
             let mut local2: &Bitmap = &bitmap1;
            Rectangle rectangle1 = Rectangle::new(0, 8, 28, 16);
            let mut srcrect1: &Rectangle = &rectangle1
            Rectangle rectangle2 = Rectangle::new(x, y + 8, width, num17 - 16);
            let mut destrect1: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
             let mut local3: &Graphics = &Expression;
            bitmap2: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
             let mut local4: &Bitmap = &bitmap2;
            rectangle2 = Rectangle::new(0, 0, 28, 8);
            let mut srcrect2: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(x, y, width, 8);
            let mut destrect2: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
             let mut local5: &Graphics = &Expression;
            bitmap3: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
             let mut local6: &Bitmap = &bitmap3;
            rectangle2 = Rectangle::new(0, 28, 28, 8);
            let mut srcrect3: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(x, y + num17 - 8, width, 8);
            let mut destrect3: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
          }
          else
          {
            Rectangle rectangle3;
            Rectangle rectangle4;
            if (self.TabCount == 0)
            {
               let mut local7: &Graphics = &Expression;
              bitmap4: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
               let mut local8: &Bitmap = &bitmap4;
              rectangle3 = Rectangle::new(0, 2, 20, 6);
              let mut srcrect4: &Rectangle = &rectangle3
              rectangle4 = Rectangle::new(x, 2 + self.ItemSize, 20, self.Height - 4);
              let mut destrect4: &Rectangle = &rectangle4
              DrawMod.DrawSimplePart2( local7,  local8, srcrect4, destrect4);
               let mut local9: &Graphics = &Expression;
              bitmap5: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
               let mut local10: &Bitmap = &bitmap5;
              rectangle3 = Rectangle::new(0, 0, 20, 2);
              let mut srcrect5: &Rectangle = &rectangle3
              rectangle4 = Rectangle::new(x, self.ItemSize, 20, 2);
              let mut destrect5: &Rectangle = &rectangle4
              DrawMod.DrawSimplePart2( local9,  local10, srcrect5, destrect5);
               let mut local11: &Graphics = &Expression;
              bitmap6: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
               let mut local12: &Bitmap = &bitmap6;
              rectangle3 = Rectangle::new(0, 8, 20, 2);
              let mut srcrect6: &Rectangle = &rectangle3
              rectangle4 = Rectangle::new(x, self.Height + self.ItemSize - 2, 20, 2);
              let mut destrect6: &Rectangle = &rectangle4
              DrawMod.DrawSimplePart2( local11,  local12, srcrect6, destrect6);
            }
            else
            {
               let mut local13: &Graphics = &Expression;
              bitmap7: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
               let mut local14: &Bitmap = &bitmap7;
              rectangle3 = Rectangle::new(0, 2, 20, 6);
              let mut srcrect7: &Rectangle = &rectangle3
              rectangle4 = Rectangle::new(x, 2 + self.ItemSize, 20, self.Height - (4 + self.ItemSize));
              let mut destrect7: &Rectangle = &rectangle4
              DrawMod.DrawSimplePart2( local13,  local14, srcrect7, destrect7);
               let mut local15: &Graphics = &Expression;
              bitmap8: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
               let mut local16: &Bitmap = &bitmap8;
              rectangle3 = Rectangle::new(0, 0, 20, 2);
              let mut srcrect8: &Rectangle = &rectangle3
              rectangle4 = Rectangle::new(x, self.ItemSize, 20, 2);
              let mut destrect8: &Rectangle = &rectangle4
              DrawMod.DrawSimplePart2( local15,  local16, srcrect8, destrect8);
               let mut local17: &Graphics = &Expression;
              bitmap9: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
               let mut local18: &Bitmap = &bitmap9;
              rectangle3 = Rectangle::new(0, 8, 20, 2);
              let mut srcrect9: &Rectangle = &rectangle3
              rectangle4 = Rectangle::new(x, self.Height - 2, 20, 2);
              let mut destrect9: &Rectangle = &rectangle4
              DrawMod.DrawSimplePart2( local17,  local18, srcrect9, destrect9);
            }
             let mut local19: &Graphics = &Expression;
            bitmap10: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
             let mut local20: &Bitmap = &bitmap10;
            rectangle3 = Rectangle::new(0, 2, 20, 6);
            let mut srcrect10: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(x, y + 2, width, num16 - 2);
            let mut destrect10: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local19,  local20, srcrect10, destrect10);
             let mut local21: &Graphics = &Expression;
            bitmap11: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
             let mut local22: &Bitmap = &bitmap11;
            rectangle3 = Rectangle::new(0, 0, 20, 2);
            let mut srcrect11: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(x, y, width, 2);
            let mut destrect11: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local21,  local22, srcrect11, destrect11);
             let mut local23: &Graphics = &Expression;
            bitmap12: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
             let mut local24: &Bitmap = &bitmap12;
            rectangle3 = Rectangle::new(0, 8, 20, 2);
            let mut srcrect12: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(x, y + num16 - 2, 10, 2);
            let mut destrect12: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local23,  local24, srcrect12, destrect12);
          }
        }
      }
      if (self.MarcStyle)
      {
        if (self.TabCount > 0)
          DrawMod.DrawFrame( self.OwnBitmap,  self.backbitmap,  Expression, 0, self.ItemSize, self.Width - 10, self.Height - self.ItemSize, -1, -1);
        else
          DrawMod.DrawFrame( self.OwnBitmap,  self.backbitmap,  Expression, 0, self.ItemSize, self.Width - 10, self.Height, -1, -1);
      }
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return self.OwnBitmap;
    }

    pub fn HandleMouseUp(x: i32, y: i32) -> i32
    {
      if (!(self.clickscroll == 1 | self.Scroller & !self.BlockScroller))
        return -1;
      self.clickscroll = 0;
      self.Scroller = false;
      return 1;
    }

    pub fn Click(x: i32, y: i32, let mut b: i32 = 1) -> i32
    {
      let mut num1: i32 = y;
      y =  Math.Round(Conversion.Int( y /  self.ItemSize));
      if (y >= 0 & y <= self.ListSize + 1)
      {
        if (self.TabCount > 0 & num1 < self.ItemSize)
        {
          if (!self.MarcStyle)
          {
            let mut num2: i32 = Math.Min(120,  Math.Round( self.Width * 0.9 /  (self.TabCount + 1)));
            let mut num3: i32 =  Math.Round(Conversion.Int( x /  num2));
            if (num3 <= self.TabCount)
              self.Tab = num3;
            self.TopItem = 0;
            return -1;
          }
          float tabCount =  self.TabCount;
          for (float a = 0.0f;  a <=  tabCount; a += 1)
          {
            if (x >= self.TabRect[ Math.Round( a)].X & y >= self.TabRect[ Math.Round( a)].Y && x <= self.TabRect[ Math.Round( a)].X + self.TabRect[ Math.Round( a)].Width & y <= self.TabRect[ Math.Round( a)].Y + self.TabRect[ Math.Round( a)].Height)
            {
              self.Tab =  Math.Round( a);
              self.TopItem = 0;
              return -1;
            }
          }
        }
        else
        {
          if (x < self.Width - 30)
          {
            y += self.TopItem;
            if (y > self.ListObj[self.Tab].ListCount)
              return -1;
            self.ListSelect = y;
            self.clickscroll = 0;
            return self.ListObj[self.Tab].ListData[self.ListSelect];
          }
          if (!self.BlockScroller)
          {
            self.clickscroll = 1;
            self.Scroller = true;
            num4: i32;
            num5: i32;
            if (self.TabCount > 0)
            {
              num4 = (self.ListSize + 0) * self.ItemSize;
              num5 = num1 - self.ItemSize * 2;
            }
            else
            {
              num4 = (self.ListSize + 1) * self.ItemSize;
              num5 = num1 - self.ItemSize * 1;
            }
            if (num5 < 1)
              num5 = 1;
            let mut num6: i32 =  Math.Round(  Math.Round( ( num5 /  num4 *  self.ListObj[self.Tab].ListCount)) -  self.ListSize / 2.0);
            if (0 > num6)
              num6 = 0;
            self.TopItem = num6;
            if (self.TabCount > 0)
            {
              if (self.TopItem > self.ListObj[self.Tab].ListCount - (self.ListSize - 1))
                self.TopItem = self.ListObj[self.Tab].ListCount - (self.ListSize - 1);
            }
            else if (self.TopItem > self.ListObj[self.Tab].ListCount - self.ListSize)
              self.TopItem = self.ListObj[self.Tab].ListCount - self.ListSize;
            if (0 > self.TopItem)
              self.TopItem = 0;
            return -1;
          }
        }
      }
      num7: i32;
      return num7;
    }

    pub bool MouseMove(x: i32, y: i32)
    {
      if (self.BlockScroller)
        return false;
      let mut num1: i32 = y;
      y =  Math.Round(Conversion.Int( y /  self.ItemSize));
      if (!(y >= 0 & y <= self.ListSize & self.clickscroll == 1))
        return false;
      num2: i32;
      num3: i32;
      if (self.TabCount > 0)
      {
        num2 = (self.ListSize + 0) * self.ItemSize;
        num3 = num1 - self.ItemSize * 2;
      }
      else
      {
        num2 = (self.ListSize + 1) * self.ItemSize;
        num3 = num1 - self.ItemSize * 1;
      }
      if (num3 < 1)
        num3 = 1;
      let mut num4: i32 =  Math.Round(  Math.Round( ( num3 /  num2 *  self.ListObj[self.Tab].ListCount)) -  self.ListSize / 2.0);
      if (0 > num4)
        num4 = 0;
      self.TopItem = num4;
      if (self.TabCount > 0)
      {
        if (self.TopItem > self.ListObj[self.Tab].ListCount - (self.ListSize - 1))
          self.TopItem = self.ListObj[self.Tab].ListCount - (self.ListSize - 1);
      }
      else if (self.TopItem > self.ListObj[self.Tab].ListCount - self.ListSize)
        self.TopItem = self.ListObj[self.Tab].ListCount - self.ListSize;
      if (0 > self.TopItem)
        self.TopItem = 0;
      return true;
    }
  }
}
