// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UDSMatrixSubPartClass
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
  pub class UDSMatrixSubPartClass : SubPartClass
  {
     ListSize: i32;
     ListSelect: i32;
     ColSelect: i32;
    pub TopItemX: i32;
    pub TopItemY: i32;
     StringListClass ListObj;
     OwnFont: Font;
     ownfont2: Font;
     ItemSize: i32;
     const let mut ItemFontOffset: i32 = 1;
     const let mut LeftTextOffset: i32 = 5;
     Width: i32;
     Height: i32;
     game: GameClass;
     bool Highlight;
     bx: i32;
     by: i32;
     backbitmap: Bitmap;
     clickscroll: i32;
     rowheight: i32;
     fontsize: i32;
     fontoffsety: i32;
     bool nolines;
     bool Marcy;
     colWidth: i32;
     minimumColWidth: i32;
     totalColWidth: i32;
     twoColumnVariant: i32;
     slotRegimes: i32;
     slotPortrait: i32;
     slotCharacter: i32;

    pub fn SubDispose()
    {
      if (Information.IsNothing( self.backbitmap))
        return;
      self.backbitmap.Dispose();
      self.backbitmap = (Bitmap) null;
    }

    pub fn ShiftUp()
    {
      --self.ListSelect;
      if (self.ListSelect - self.TopItemY >= 0)
        return;
      --self.TopItemY;
    }

    pub fn ShiftDown()
    {
      self += 1.ListSelect;
      if (self.ListSelect - self.TopItemY <= self.ListSize)
        return;
      self += 1.TopItemY;
    }

    pub fn ShiftLeft()
    {
      --self.ColSelect;
      if (self.ColSelect - self.TopItemX >= 0)
        return;
      --self.TopItemX;
    }

    pub fn ShiftRight()
    {
      let mut num1: i32 =  Math.Round(Conversion.Int( (self.Width - 0) /  (self.ListObj.Width + 1)));
      if (num1 < self.colWidth)
        num1 = self.colWidth;
      let mut num2: i32 =  Math.Round( (self.Width - 0) /  num1) - 2;
      if (num2 < 0)
        num2 = 0;
      self += 1.ColSelect;
      if (self.ColSelect - self.TopItemX <= num2)
        return;
      self += 1.TopItemX;
    }

    pub fn Refresh(StringListClass tListObj, tlistselect: i32, tcolselect: i32)
    {
      self.ListObj = tListObj;
      self.ListSelect = tlistselect;
      self.ColSelect = tcolselect;
      if (self.TopItemY > self.ListObj.Length - self.ListSize)
        self.TopItemY = self.ListObj.Length - self.ListSize;
      if (0 > self.TopItemY)
        self.TopItemY = 0;
      self.Clear();
    }

    pub fn DescriptInfo(x: i32, y: i32)
    {
      if (self.game.Data.Round < 1)
        return;
      if (x > 0 & y > 0 & x < self.Width & y < self.Height)
      {
        Coordinate coordinate = self.Click2(x, y, 0);
        if (coordinate.x >= 0 & coordinate.y >= 0 & coordinate.y <= self.ListObj.TempDesc.GetUpperBound(1) & coordinate.x <= self.ListObj.TempDesc.GetUpperBound(0))
          self.Descript = self.ListObj.TempDesc[coordinate.x, coordinate.y];
        else
          self.Descript = "";
      }
      else
        self.Descript = "";
    }

    pub UDSMatrixSubPartClass(
      StringListClass tListobj,
      tlistsize: i32,
      twidth: i32,
      tlistselect: i32,
      tcolselect: i32,
      tgame: GameClass,
      bool systemfont = false,
      bool tHighlight = true,
      let mut tTop: i32 = 0,
       tbackbitmap: Bitmap = null,
      let mut bbx: i32 = -1,
      let mut bby: i32 = -1,
      let mut trowheight: i32 = 16,
      let mut tfontsize: i32 = 12,
      let mut tfontoffsety: i32 = 0,
      bool tnolines = false,
      bool tMarcy = false,
      let mut tMinColValue: i32 = -1,
      let mut tTwoColumnVariant: i32 = 0,
       customFont: Font = null)
      : base(twidth, (tlistsize + 3) * trowheight)
    {
      self.nolines = tnolines;
      if (!Information.IsNothing( tbackbitmap))
      {
        if (!Information.IsNothing( self.backbitmap))
          self.backbitmap.Dispose();
        self.backbitmap = new Bitmap(self.OwnBitmap.Width, self.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
        self.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics graphics = Graphics.FromImage((Image) self.backbitmap);
        graphics.CompositingMode = CompositingMode.SourceCopy;
        graphics.DrawImage((Image) tbackbitmap, Rectangle::new(0, 0, self.OwnBitmap.Width, self.OwnBitmap.Height), Rectangle::new(bbx, bby, self.OwnBitmap.Width, self.OwnBitmap.Height), GraphicsUnit.Pixel);
        graphics.CompositingMode = CompositingMode.SourceOver;
        graphics.Dispose();
      }
      libName: String = "SE_Data";
      self.slotPortrait = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(libName, 204, 0, 0));
      self.slotRegimes = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(libName, 143, 0, 0));
      self.slotCharacter = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(libName, 196, 0, 0));
      self.bx = bbx;
      self.by = bby;
      self.Marcy = tMarcy;
      self.ItemSize = trowheight;
      self.fontsize = tfontsize;
      self.fontoffsety = tfontoffsety;
      self.Width = twidth;
      self.Height = (tlistsize + 3) * self.ItemSize;
      self.ListSize = tlistsize;
      self.ListSelect = tlistselect;
      self.ListObj = tListobj;
      self.MouseOver = true;
      self.ColSelect = tcolselect;
      self.clickscroll = 0;
      self.Highlight = tHighlight;
      self.TopItemY = tTop;
      self.twoColumnVariant = tTwoColumnVariant;
      self.game = tgame;
      if (tTop == 0)
      {
        self.TopItemY =  Math.Round( self.ListSelect - Conversion.Int( self.ListSize / 2.0));
        if (self.TopItemY < 0)
          self.TopItemY = 0;
      }
      if ( Math.Round(Conversion.Int( (self.Width - 0) /  (self.ListObj.Width + 1))) < self.colWidth)
      {
        let mut colWidth: i32 = self.colWidth;
        if (self.ColSelect > 0)
        {
          self.TopItemX =  Math.Round( self.ColSelect - Conversion.Int( self.ColSelect / 2.0));
          if (self.TopItemX < 0)
            self.TopItemX = 0;
        }
      }
      else
        self.TopItemX = 0;
      if (!Information.IsNothing( customFont))
      {
        self.OwnFont = customFont;
        self.ownfont2 = customFont;
      }
      else if (!systemfont)
      {
        if (self.Marcy)
        {
          if (self.game.Data.Round == 0)
          {
            self.OwnFont = self.game.MarcFont5;
            self.ownfont2 = self.game.MarcFont7;
          }
          else
          {
            self.OwnFont = self.game.MarcFont4;
            self.ownfont2 = self.game.MarcFont4;
          }
        }
        else
        {
          self.OwnFont = Font::new(self.game.FontCol.Families[1],  self.fontsize, FontStyle.Regular, GraphicsUnit.Pixel);
          self.ownfont2 = Font::new(self.game.FontCol.Families[1],   Math.Round( self.fontsize * 0.9), FontStyle.Regular, GraphicsUnit.Pixel);
        }
      }
      else
      {
        self.OwnFont = Font::new("Courier New", 12f, FontStyle.Regular, GraphicsUnit.Pixel);
        self.ownfont2 = Font::new("Courier New", 11f, FontStyle.Regular, GraphicsUnit.Pixel);
      }
      self.minimumColWidth = 30;
      if (tMinColValue > 0)
        self.minimumColWidth = tMinColValue;
      self.colWidth =  Math.Round(Conversion.Int( (self.Width - 0) /  (self.ListObj.Width + 1)));
      if (self.minimumColWidth > self.colWidth)
      {
        self.colWidth = self.minimumColWidth;
        --self.ListSize;
      }
      self.totalColWidth = self.colWidth * (self.ListObj.Width + 1);
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Color.FromArgb(0,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Color.FromArgb(0,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      if (self.ListSize >= self.ListObj.Length)
        self.TopItemY = 0;
      if (Information.IsNothing( self.backbitmap))
      {
        graphics.Clear(Color.Transparent);
      }
      else
      {
        graphics.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( graphics,  self.backbitmap, 0, 0);
        graphics.CompositingMode = CompositingMode.SourceOver;
      }
      let mut num1: i32 =  Math.Round(( self.ItemSize -  graphics.MeasureString("X", self.OwnFont).Height) / 2.0) - 1;
      if (num1 < 0)
        num1 = 0;
      let mut num2: i32 =  Math.Round(( self.ItemSize -  graphics.MeasureString("X", self.ownfont2).Height) / 2.0) - 1;
      if (num2 < 0)
        num1 = 0;
      DrawMod.DrawBlockGradient2( graphics, 0, 0, self.Width - 0, self.ItemSize, Color.FromArgb( byte.MaxValue, 0, 0, 0), Color.FromArgb(200, 0, 0, 0));
      if (!self.nolines)
        DrawMod.DrawBlock( graphics, 0, self.ItemSize, self.Width - 1, self.Height - 2 * self.ItemSize, 0, 0, 0, 20);
      let mut num3: i32 = 0;
      if (self.ListSize >= self.ListObj.Length)
        num3 = 0;
      let mut num4: i32 = 2;
      let mut num5: i32 = 1;
      let mut num6: i32 = -1;
      let mut topItemY: i32 = self.TopItemY;
      let mut num7: i32 = self.TopItemY + self.ListSize + num4;
      for (let mut index1: i32 = topItemY; index1 <= num7; index1 += 1)
      {
        num6 += 1;
        bitmap: Bitmap;
        if (num6 == 0)
        {
          if (self.ListObj.Width > -1)
          {
            if (self.ListObj.Width > self.ListObj.ColumnName.GetUpperBound(0))
              self.ListObj.ColumnName = (string[]) Utils.CopyArray((Array) self.ListObj.ColumnName, (Array) new string[self.ListObj.Width + 1]);
            let mut width: i32 = self.ListObj.Width;
            for (let mut index2: i32 = 0; index2 <= width; index2 += 1)
            {
              if (Operators.CompareString(self.ListObj.ColumnName[index2], (string) null, false) == 0)
                self.ListObj.ColumnName[index2] = "";
              let mut num8: i32 =  Math.Round(Conversion.Int( (self.Width - 0) /  (self.ListObj.Width + 1)));
              if (num8 < self.colWidth)
                num8 = self.colWidth;
              if (self.ListObj.ColWidth[index2] > 0)
                num8 = self.ListObj.ColWidth[index2];
              let mut x1: i32 = 5 + num8 * index2 - num8 * self.TopItemX;
              if (self.ListObj.ColWidth[index2] > 0)
              {
                x1 = 5;
                let mut num9: i32 = index2 - 1;
                for (let mut index3: i32 = 0; index3 <= num9; index3 += 1)
                  x1 += self.ListObj.ColWidth[index3];
              }
              if (self.twoColumnVariant > 0 && index2 == 1)
                x1 = self.twoColumnVariant + 5;
              if (x1 > 0 & x1 <= self.Width)
              {
                if (Conversions.ToDouble(self.ListObj.TempColumBmp[index2]) > 0.0)
                {
                   let mut local1: &Graphics = &graphics;
                  bitmap = BitmapStore.GetBitmap(Conversions.ToInteger(self.ListObj.TempColumBmp[index2]));
                   let mut local2: &Bitmap = &bitmap;
                  let mut x2: i32 = x1;
                  let mut y: i32 = self.ItemSize * num6 + 1 + 1;
                  DrawMod.DrawSimple( local1,  local2, x2, y);
                }
                else
                {
                  str: String = self.ListObj.ColumnName[index2];
                  if (Strings.InStr(str, "\r\n") > 0)
                    DrawMod.DrawTextColouredNicely( graphics, str, self.OwnFont, x1, self.ItemSize * num6 + 1 + self.fontoffsety + 4, Color.White);
                  else
                    DrawMod.DrawTextColouredNicely( graphics, str, self.OwnFont, x1, self.ItemSize * num6 + 1 + num1 + self.fontoffsety + 2, Color.White);
                }
              }
            }
          }
        }
        else if (num6 != self.ListSize + 2 && index1 - num5 <= self.ListObj.Length && self.ListObj.Width > -1)
        {
          if (!self.nolines & index1 != self.TopItemY + self.ListSize + num4)
            DrawMod.drawLine( graphics, 0, self.ItemSize * num6 + self.ItemSize, self.Width - 2, self.ItemSize * num6 + self.ItemSize, 128, 128, 128,  byte.MaxValue, 2);
          let mut width1: i32 = self.ListObj.Width;
          for (let mut index4: i32 = 0; index4 <= width1; index4 += 1)
          {
            let mut num10: i32 =  Math.Round(Conversion.Int( (self.Width - 0) /  (self.ListObj.Width + 1)));
            if (num10 < self.colWidth)
              num10 = self.colWidth;
            if (self.ListObj.ColWidth[index4] > 0)
              num10 = self.ListObj.ColWidth[index4];
            let mut x3: i32 = 5 + num10 * index4 - num10 * self.TopItemX;
            if (self.ListObj.ColWidth[index4] > 0)
            {
              x3 = 5;
              let mut num11: i32 = index4 - 1;
              for (let mut index5: i32 = 0; index5 <= num11; index5 += 1)
                x3 += self.ListObj.ColWidth[index5];
            }
            if (self.twoColumnVariant > 0)
            {
              if (index4 == 0)
                num10 = self.twoColumnVariant;
              if (index4 == 1)
              {
                x3 = self.twoColumnVariant + 5;
                num10 = self.Width - -self.twoColumnVariant;
              }
            }
            if (x3 > 0 & x3 <= self.Width)
            {
              if (!(index4 == 0 & !self.Marcy) || self.ListSelect != index1 - num5)
                ;
              if (self.ListSelect == index1 - num5 & self.ColSelect == index4)
              {
                if (self.nolines || !self.Marcy)
                  ;
              }
              else if (self.game.Data.Product >= 5 && !self.Marcy & self.ColSelect > -1 & self.ColSelect <= self.ListObj.Width & self.ListSelect > -1)
              {
                str1: String = self.ListObj.Data[self.ListSelect, self.ColSelect];
              }
              strArray1: Vec<String> = self.ListObj.Data[index1 - num5, index4].Split('#');
              if (strArray1.GetUpperBound(0) > 0)
              {
                SizeF sizeF2;
                if (Operators.CompareString(strArray1[0], "1", false) == 0)
                {
                  let mut num12: i32 = 0;
                  sizeF2 = graphics.MeasureString(strArray1[1], self.ownfont2);
                  DrawMod.DrawTextColouredNicely( graphics, strArray1[1], self.ownfont2, x3, self.ItemSize * num6 + 1 + 3 + num2 + self.fontoffsety, Color.Black);
                  let mut num13: i32 =  Math.Round( ( num12 + sizeF2.Width));
                  let mut nr: i32 =  Math.Round(Conversion.Val(strArray1[2]));
                  let mut width2: i32 = BitmapStore.GetWidth(nr);
                  let mut num14: i32 = BitmapStore.Getheight(nr);
                   let mut local3: &Graphics = &graphics;
                  bitmap = BitmapStore.GetBitmap(nr);
                   let mut local4: &Bitmap = &bitmap;
                  let mut x4: i32 = x3 + num13;
                  let mut y: i32 = self.ItemSize * num6 +  Math.Round( (self.ItemSize - num14) / 2.0) + 1;
                  let mut w: i32 = width2;
                  let mut h: i32 = num14;
                  DrawMod.DrawScaled( local3,  local4, x4, y, w, h);
                  if (strArray1.GetUpperBound(0) > 2)
                  {
                    let mut num15: i32 = num13 + width2;
                    DrawMod.DrawTextColouredNicely( graphics, strArray1[3], self.ownfont2, x3 + num15, self.ItemSize * num6 + 1 + 3 + num2 + self.fontoffsety, Color.Black);
                  }
                }
                if (Operators.CompareString(strArray1[0], "99", false) == 0)
                {
                  let mut num16: i32 = 0;
                  let mut upperBound: i32 = strArray1.GetUpperBound(0);
                  for (let mut index6: i32 = 1; index6 <= upperBound; index6 += 1)
                  {
                    let mut nr: i32 =  Math.Round(Conversion.Val(strArray1[index6]));
                    if (nr > 0)
                    {
                       let mut local5: &Graphics = &graphics;
                      bitmap = BitmapStore.GetBitmap(nr);
                       let mut local6: &Bitmap = &bitmap;
                      let mut x5: i32 = x3 + num16;
                      let mut y: i32 = self.ItemSize * num6 +  Math.Round( (self.ItemSize - 18) / 2.0);
                      DrawMod.DrawSimple( local5,  local6, x5, y);
                    }
                    num16 += 20;
                    if (num16 > num10 - 20)
                      break;
                  }
                }
                if (Operators.CompareString(strArray1[0], "2", false) == 0)
                {
                  let mut num17: i32 = 0;
                  let mut num18: i32 = 0;
                  if (strArray1.GetUpperBound(0) >= 1)
                  {
                    if (strArray1.GetUpperBound(0) <= 2)
                      num18 = -12;
                    let mut upperBound: i32 = strArray1.GetUpperBound(0);
                    for (let mut index7: i32 = 1; index7 <= upperBound; index7 += 2)
                    {
                      let mut nr1: i32 =  Math.Round(Conversion.Val(strArray1[index7]));
                      let mut nr2: i32 = 0;
                      if (strArray1.GetUpperBound(0) >= 2)
                        nr2 =  Math.Round(Conversion.Val(strArray1[index7 + 1]));
                      if (nr1 > 0)
                      {
                         let mut local7: &Graphics = &graphics;
                        bitmap = BitmapStore.GetBitmap(nr1);
                         let mut local8: &Bitmap = &bitmap;
                        let mut x6: i32 = x3 + num17;
                        let mut y: i32 = self.ItemSize * num6 +  Math.Round( (self.ItemSize - 20) / 2.0) + num18;
                        DrawMod.DrawSimple( local7,  local8, x6, y);
                      }
                      if (nr2 > 0)
                      {
                         let mut local9: &Graphics = &graphics;
                        bitmap = BitmapStore.GetBitmap(nr2);
                         let mut local10: &Bitmap = &bitmap;
                        let mut x7: i32 = x3 + num17;
                        let mut y: i32 = self.ItemSize * num6 +  Math.Round( (self.ItemSize - 20) / 2.0) + num18;
                        DrawMod.DrawSimple( local9,  local10, x7, y);
                      }
                      num17 += 20;
                      if (index7 == 1)
                      {
                        num17 += 6;
                        num18 = -12;
                      }
                      if (num17 > num10 - 20)
                        break;
                    }
                  }
                }
                if (Operators.CompareString(strArray1[0], "3", false) == 0)
                {
                  let mut num19: i32 = 5;
                  let mut upperBound: i32 = strArray1.GetUpperBound(0);
                  for (let mut index8: i32 = 1; index8 <= upperBound; index8 += 2)
                  {
                    let mut nr: i32 =  Math.Round(Conversion.Val(strArray1[index8]));
                    if (nr > 0)
                    {
                       let mut local11: &Graphics = &graphics;
                      bitmap = BitmapStore.GetBitmap(nr);
                       let mut local12: &Bitmap = &bitmap;
                      let mut x8: i32 = x3 + num19 - 6;
                      let mut y: i32 = self.ItemSize * num6 +  Math.Round( (self.ItemSize - BitmapStore.Getheight(nr)) / 2.0);
                      DrawMod.DrawSimple( local11,  local12, x8, y);
                    }
                    let mut num20: i32 = num19 + 32;
                    sizeF2 = graphics.MeasureString(strArray1[index8 + 1], self.ownfont2);
                    DrawMod.DrawTextColouredNicely( graphics, strArray1[index8 + 1], self.ownfont2, x3 + num20, self.ItemSize * num6 + 1 - 1 + num2 + self.fontoffsety, Color.Black);
                    num19 =  Math.Round( ( num20 + (sizeF2.Width + 10f)));
                    if (num19 > num10 - 45)
                      break;
                  }
                }
                if (Operators.CompareString(strArray1[0], "6", false) == 0)
                {
                  let mut num21: i32 = 5;
                  let mut upperBound: i32 = strArray1.GetUpperBound(0);
                  for (let mut index9: i32 = 1; index9 <= upperBound; index9 += 5)
                  {
                    let mut num22: i32 =  Math.Round(Conversion.Val(strArray1[index9]));
                    objBitmap: Bitmap = self.game.CustomBitmapObj.DrawSFTypeGraphic(self.game.HandyFunctionsObj.GetSFTypeByID(num22),  Math.Round(Conversion.Val(strArray1[index9 + 2])) == 1,  Math.Round(Conversion.Val(strArray1[index9 + 3])),  Math.Round(Conversion.Val(strArray1[index9 + 4])), -1);
                    let mut num23: i32 = 0;
                    let mut num24: i32 = 2;
                    let mut w: i32 = 64;
                    let mut h: i32 = self.ItemSize;
                    let mut width3: i32 = objBitmap.Width;
                    let mut height: i32 = objBitmap.Height;
                    if (width3 > w | height > h)
                    {
                      if ( width3 /  w >  height /  h)
                      {
                        float num25 =  w /  width3;
                        float num26 =  h -  height * num25;
                        let mut num27: i32 = num24 +  Math.Round( (num26 / 2f));
                        h =  Math.Round( ( h - num26));
                      }
                      else
                      {
                        float num28 =  h /  height;
                        float num29 =  w -  width3 * num28;
                        let mut num30: i32 = num23 +  Math.Round( (num29 / 2f));
                        w =  Math.Round( ( w - num29));
                      }
                      DrawMod.DrawScaled( graphics,  objBitmap, x3 + num21 - 6, self.ItemSize * num6 + 0, w, h, true);
                    }
                    else
                      DrawMod.DrawSimple( graphics,  objBitmap, x3 + num21 - 6, self.ItemSize * num6 +  Math.Round( (self.ItemSize - BitmapStore.Getheight(num22)) / 2.0));
                    let mut num31: i32 = num21 + w;
                    sizeF2 = graphics.MeasureString(strArray1[index9 + 1], self.ownfont2);
                    DrawMod.DrawTextColouredNicely( graphics, strArray1[index9 + 1], self.ownfont2, x3 + num31, self.ItemSize * num6 + 1 - 1 + num2 + self.fontoffsety, Color.Black);
                    num21 =  Math.Round( ( num31 + (sizeF2.Width + 10f)));
                    if (num21 > num10 - 45)
                      break;
                  }
                }
                if (Operators.CompareString(strArray1[0], "4", false) == 0)
                {
                  let mut num32: i32 = 0;
                  let mut upperBound: i32 = strArray1.GetUpperBound(0);
                  for (let mut index10: i32 = 1; index10 <= upperBound; index10 += 2)
                  {
                    let mut nr: i32 =  Math.Round(Conversion.Val(strArray1[index10]));
                    if (nr > 0)
                    {
                       let mut local13: &Graphics = &graphics;
                      bitmap = BitmapStore.GetBitmap(nr, -1);
                       let mut local14: &Bitmap = &bitmap;
                      let mut x9: i32 = x3 + num32 - 6;
                      let mut y: i32 = self.ItemSize * num6 +  Math.Round( (self.ItemSize - BitmapStore.Getheight(nr, -1)) / 2.0);
                      DrawMod.DrawSimple( local13,  local14, x9, y);
                    }
                    let mut num33: i32 = num32 + 20;
                    sizeF2 = graphics.MeasureString(strArray1[index10 + 1], self.ownfont2);
                    DrawMod.DrawTextColouredNicely( graphics, strArray1[index10 + 1], self.ownfont2, x3 + num33, self.ItemSize * num6 + 1 - 1 + num2 + self.fontoffsety, Color.Black);
                    num32 =  Math.Round( ( num33 + sizeF2.Width));
                    if (num32 > num10 - 45)
                      break;
                  }
                }
                if (Operators.CompareString(strArray1[0], "5", false) == 0)
                {
                  let mut num34: i32 = 3;
                  let mut upperBound: i32 = strArray1.GetUpperBound(0);
                  for (let mut index11: i32 = 1; index11 <= upperBound; index11 += 2)
                  {
                    let mut nr: i32 =  Math.Round(Conversion.Val(strArray1[index11]));
                    if (nr > 0)
                    {
                       let mut local15: &Graphics = &graphics;
                      bitmap = BitmapStore.GetBitmap(nr);
                       let mut local16: &Bitmap = &bitmap;
                      let mut x10: i32 = x3 + num34 - 7;
                      let mut y: i32 = self.ItemSize * num6 +  Math.Round( (self.ItemSize - BitmapStore.Getheight(nr)) / 2.0);
                      DrawMod.DrawSimple( local15,  local16, x10, y);
                    }
                    let mut num35: i32 = num34 + 22;
                    sizeF2 = graphics.MeasureString(strArray1[index11 + 1], self.ownfont2);
                    DrawMod.DrawTextColouredNicely( graphics, strArray1[index11 + 1], self.ownfont2, x3 + num35, self.ItemSize * num6 + 1 + 1 + num2 + self.fontoffsety, Color.Black);
                    num34 =  Math.Round( ( num35 + (sizeF2.Width + 8f)));
                    if (num34 > num10 - 45)
                      break;
                  }
                }
                if (Operators.CompareString(strArray1[0], "5b", false) == 0)
                {
                  let mut num36: i32 = 3;
                  let mut num37: i32 = 1;
                  let mut num38: i32 = 0;
                  let mut upperBound: i32 = strArray1.GetUpperBound(0);
                  for (let mut index12: i32 = 1; index12 <= upperBound; index12 += 2)
                  {
                    let mut nr: i32 =  Math.Round(Conversion.Val(strArray1[index12]));
                    let mut num39: i32 = 0;
                    if (nr > 0)
                      num39 =  Math.Round( (self.ItemSize - BitmapStore.Getheight(nr) * 2) / 3.0);
                    if (nr > 0 & num37 == 1)
                    {
                       let mut local17: &Graphics = &graphics;
                      bitmap = BitmapStore.GetBitmap(nr);
                       let mut local18: &Bitmap = &bitmap;
                      let mut x11: i32 = x3 + num36 - 7;
                      let mut y: i32 = self.ItemSize * num6 + num39;
                      DrawMod.DrawSimple( local17,  local18, x11, y);
                    }
                    if (nr > 0 & num37 == 2)
                    {
                       let mut local19: &Graphics = &graphics;
                      bitmap = BitmapStore.GetBitmap(nr);
                       let mut local20: &Bitmap = &bitmap;
                      let mut x12: i32 = x3 + num36 - 7;
                      let mut y: i32 = self.ItemSize * num6 + BitmapStore.Getheight(nr) + num39 * 2;
                      DrawMod.DrawSimple( local19,  local20, x12, y);
                    }
                    sizeF2 = graphics.MeasureString(strArray1[index12 + 1], self.ownfont2);
                    if ( sizeF2.Width >  num38)
                      num38 =  Math.Round( sizeF2.Width);
                    if (num37 == 1)
                      DrawMod.DrawTextColouredNicely( graphics, strArray1[index12 + 1], self.ownfont2, x3 + num36 + 22, self.ItemSize * num6 + num39, Color.Black);
                    if (num37 == 2)
                      DrawMod.DrawTextColouredNicely( graphics, strArray1[index12 + 1], self.ownfont2, x3 + num36 + 22, self.ItemSize * num6 + BitmapStore.Getheight(nr) + num39 * 2, Color.Black);
                    num37 += 1;
                    if (num37 > 2)
                    {
                      num37 = 1;
                      num36 = num36 + 22 + (num38 + 8);
                      num38 = 0;
                    }
                    if (num36 > num10 - 45)
                      break;
                  }
                }
              }
              else if (self.ListObj.TempBmp[index1 - num5, index4] > 0)
              {
                let mut nr: i32 = self.ListObj.TempBmp[index1 - num5, index4];
                if (nr < 100000)
                {
                  let mut width4: i32 = BitmapStore.GetWidth(nr);
                  let mut num40: i32 = BitmapStore.Getheight(nr);
                   let mut local21: &Graphics = &graphics;
                  bitmap = BitmapStore.GetBitmap(nr);
                   let mut local22: &Bitmap = &bitmap;
                  let mut x13: i32 = x3;
                  let mut y: i32 = self.ItemSize * num6 + 1 + 1;
                  let mut w: i32 = width4;
                  let mut h: i32 = num40;
                  DrawMod.DrawScaled( local21,  local22, x13, y, w, h);
                }
                else
                {
                  if (nr == 100001)
                    self.Hardcoded_DrawPortrait(graphics, x3 - 5, self.ItemSize * num6,  Math.Round( (self.ItemSize * 10) / 14.0), self.ItemSize,  Math.Round(Conversion.Val(self.ListObj.Data[index1 - num5, index4])));
                  if (nr == 1000001)
                  {
                    strArray2: Vec<String> = self.ListObj.Data[index1 - num5, index4].Split(',');
                    objBitmap: Bitmap = self.game.CustomBitmapObj.DrawSFTypeGraphic(self.game.HandyFunctionsObj.GetSFTypeByID( Math.Round(Conversion.Val(strArray2[0]))),  Math.Round(Conversion.Val(strArray2[1])) == 1,  Math.Round(Conversion.Val(strArray2[2])),  Math.Round(Conversion.Val(strArray2[3])), -1);
                    let mut num41: i32 = 0;
                    let mut num42: i32 = 2;
                    let mut w: i32 = self.ItemSize * 2;
                    let mut h: i32 = self.ItemSize;
                    let mut width5: i32 = objBitmap.Width;
                    let mut height: i32 = objBitmap.Height;
                    if (width5 > w | height > h)
                    {
                      if ( width5 /  w >  height /  h)
                      {
                        float num43 =  w /  width5;
                        float num44 =  h -  height * num43;
                        num42 +=  Math.Round( (num44 / 2f));
                        h =  Math.Round( ( h - num44));
                      }
                      else
                      {
                        float num45 =  h /  height;
                        float num46 =  w -  width5 * num45;
                        num41 +=  Math.Round( (num46 / 2f));
                        w =  Math.Round( ( w - num46));
                      }
                      DrawMod.DrawScaled( graphics,  objBitmap, x3 - 5 + num41, self.ItemSize * num6 + num42, w, h);
                    }
                    else
                      DrawMod.DrawSimple( graphics,  objBitmap, x3 - 5 + num41 +  Math.Round( (w - width5) / 2.0), self.ItemSize * num6 + num42 +  Math.Round( (h - height) / 2.0));
                  }
                }
              }
              else
              {
                str2: String = self.ListObj.Data[index1 - num5, index4];
                str3: String = "";
                if ( (num2 * 2) >=  self.ItemSize / 2.0 &  graphics.MeasureString(str2, self.ownfont2).Width >  num10)
                {
                  Length: i32;
                  for (;  graphics.MeasureString(str2, self.ownfont2).Width >  (num10 - 4); str2 = Strings.Left(str2, Length))
                  {
                    Length = str2.LastIndexOf(" ");
                    if (Length > 0)
                      str3 = Strings.Right(str2, str2.Length - Length) + str3;
                    else
                      break;
                  }
                  let mut num47: i32 = self.ItemSize - 2 * num2;
                  let mut num48: i32 =  Math.Round( (self.ItemSize - 2 * num47) / 3.0);
                  if (Operators.CompareString(Strings.Left(str3, 1), " ", false) == 0)
                    str3 = Strings.Mid(str3, 2);
                  DrawMod.DrawTextColouredNicely( graphics, str2, self.ownfont2, x3, self.ItemSize * num6 + num48 + 2, Color.Black);
                  DrawMod.DrawTextColouredNicely( graphics, str3, self.ownfont2, x3, self.ItemSize * num6 + self.ItemSize - (num47 + num48 + 2), Color.Black);
                }
                else
                {
                  while ( graphics.MeasureString(str2, self.ownfont2).Width >  num10)
                    str2 =  graphics.MeasureString(str2, self.ownfont2).Width <=  num10 * 2.2 ? Strings.Left(str2, str2.Length - 1) : Strings.Left(str2,  Math.Round( str2.Length / 2.0));
                  if (Strings.InStr(str2, "\r\n") > 0)
                    DrawMod.DrawTextColouredNicely( graphics, str2, self.ownfont2, x3, self.ItemSize * num6 + 1 + self.fontoffsety, Color.Black);
                  else
                    DrawMod.DrawTextColouredNicely( graphics, str2, self.ownfont2, x3, self.ItemSize * num6 + 1 + num2 + self.fontoffsety + 2, Color.Black);
                }
              }
              if (!self.nolines & index4 > 0)
                DrawMod.drawLine( graphics, x3 - 5, self.ItemSize * num6, x3 - 5, 1 * self.ItemSize + self.ItemSize * num6 - 1, 128, 128, 128,  byte.MaxValue);
            }
          }
        }
      }
      DrawMod.DrawRectangle( graphics, 0, self.ItemSize, self.Width - 1, self.Height - self.ItemSize * 2, 0, 0, 0,  byte.MaxValue);
      if (!Information.IsNothing( graphics))
        graphics.Dispose();
      return self.OwnBitmap;
    }

    pub fn Hardcoded_DrawPortrait(Graphics g, x: i32, y: i32, w: i32, h: i32, charId: i32)
    {
      let mut num: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotCharacter].GetData(0, charId, 12)));
       let mut local1: &Graphics = &g;
      bitmap: Bitmap = self.game.CustomBitmapObj.DrawLeaderPortrait(charId, w, h);
       let mut local2: &Bitmap = &bitmap;
      let mut x1: i32 = x;
      let mut y1: i32 = y;
      DrawMod.DrawSimple( local1,  local2, x1, y1);
    }

    pub Coordinate Click2(x: i32, y: i32, let mut b: i32 = 1)
    {
      y =  Math.Round(Conversion.Int( y /  self.ItemSize));
      self.Scroller = true;
      let mut num1: i32 = 1;
      let mut num2: i32 = 0;
      if (self.ListSize >= self.ListObj.Length)
        num2 = 0;
      if (b == 1)
      {
        if (y > 0 & y < self.ListSize + 2)
        {
          if (x <= self.Width - num2)
          {
            y -= num1;
            y += self.TopItemY;
            float num3 =  Conversion.Int( (self.Width - 0) /  (self.ListObj.Width + 1));
            if ( num3 <  self.colWidth)
              num3 =  self.colWidth;
            self.clickscroll = 0;
            if (y > self.ListObj.Length)
            {
              Coordinate coordinate;
              coordinate.x = -1;
              coordinate.y = -1;
              return coordinate;
            }
            self.ListSelect = y;
            Coordinate coordinate1;
            coordinate1.x = self.ListSelect;
            if (self.twoColumnVariant > 0)
            {
              if (x <= self.twoColumnVariant)
              {
                coordinate1.y = 1;
                self.ColSelect = 1;
              }
              else
              {
                coordinate1.y = 1;
                self.ColSelect = 1;
              }
            }
            else
            {
              coordinate1.y =  Math.Round( (Conversion.Int( x / num3) +  self.TopItemX));
              self.ColSelect =  Math.Round( (Conversion.Int( x / num3) +  self.TopItemX));
            }
            return coordinate1;
          }
        }
        else
        {
          if (y == 0)
          {
            --self.TopItemY;
            self.clickscroll = 0;
            if (0 > self.TopItemY)
              self.TopItemY = 0;
            Coordinate coordinate;
            coordinate.x = -1;
            coordinate.y = -1;
            return coordinate;
          }
          self += 1.TopItemY;
          self.clickscroll = 0;
          if (self.TopItemY > self.ListObj.Length - self.ListSize)
            self.TopItemY = self.ListObj.Length - self.ListSize;
          if (0 > self.TopItemY)
            self.TopItemY = 0;
          Coordinate coordinate2;
          coordinate2.x = -1;
          coordinate2.y = -1;
          return coordinate2;
        }
      }
      Coordinate coordinate3;
      coordinate3.x = -1;
      coordinate3.y = -1;
      return coordinate3;
    }

    pub fn HandleMouseUp(x: i32, y: i32) -> i32
    {
      if (self.clickscroll == 1 | self.Scroller)
      {
        self.clickscroll = 0;
        self.Scroller = false;
        return 1;
      }
      if (self.clickscroll != 2)
        return -1;
      self.clickscroll = 0;
      self.Scroller = false;
      return 1;
    }

    pub bool MouseMove(x: i32, y: i32)
    {
      let mut num1: i32 = y;
      y =  Math.Round(Conversion.Int( y /  self.ItemSize));
      let mut num2: i32 = 0;
      let mut num3: i32 = 2;
      let mut num4: i32 = 0;
      if (self.ListSize >= self.ListObj.Length)
        num4 = 0;
      if (y > num2 & y < self.ListSize + num3 & self.clickscroll == 1)
      {
        let mut num5: i32 = (self.ListSize + 1) * self.ItemSize;
        let mut num6: i32 = num1 - self.ItemSize;
        if (num6 < 1)
          num6 = 1;
        let mut num7: i32 =  Math.Round(  Math.Round( ( num6 /  num5 *  self.ListObj.Length)) -  self.ListSize / 2.0);
        if (0 > num7)
          num7 = 0;
        self.TopItemY = num7;
        if (self.TopItemY > self.ListObj.Length - self.ListSize)
          self.TopItemY = self.ListObj.Length - self.ListSize;
        if (0 > self.TopItemY)
          self.TopItemY = 0;
        return true;
      }
      if (self.clickscroll != 2)
        ;
      return false;
    }
  }
}
