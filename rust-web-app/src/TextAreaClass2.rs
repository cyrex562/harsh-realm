// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TextAreaClass2
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
  pub class TextAreaClass2 : SubPartClass
  {
     ListSize: i32;
     ListSelect: i32;
    pub TopItem: i32;
    pub ListClass[] ListObj;
     Tab: i32;
     TabName: Vec<String>;
     TabCount: i32;
     OwnFont: Font;
     ItemSize: i32;
     Width: i32;
     Height: i32;
     game: GameClass;
     backbitmap: Bitmap;
     bx: i32;
     by: i32;
     fontcol: Color;
     fontColHigh: Color;
     clickscroll: i32;
    pub Rectangle[] TabRect;
     bool WithoutScrollbars;
    pub WithoutFrame: bool;
    pub centerit: bool;
    pub shadow: bool;
    pub useEncy: bool;
     bool minimalHeight;
    pub int[,,] encyId;
    pub darkerFrame: bool;
     StringListClass tStringList;

    pub fn SubDispose()
    {
      if (Information.IsNothing( self.backbitmap))
        return;
      self.backbitmap.Dispose();
      self.backbitmap = (Bitmap) null;
    }

    pub bool HandleTimerWheel(x: i32, y: i32,  WindowClass tWindow)
    {
      if (self.game.EditObj.MouseWheel > 0)
      {
        let mut mouseWheel: i32 = self.game.EditObj.MouseWheel;
        for (let mut index: i32 = 1; index <= mouseWheel; index += 1)
          self.ShiftUp();
        self.game.EditObj.MouseWheel = 0;
        self.game.EditObj.MouseWheelWait = 4;
        return true;
      }
      if (self.game.EditObj.MouseWheel >= 0)
        return false;
      let mut mouseWheel1: i32 = self.game.EditObj.MouseWheel;
      for (let mut index: i32 = -1; index >= mouseWheel1; index += -1)
        self.ShiftDown();
      self.game.EditObj.MouseWheel = 0;
      self.game.EditObj.MouseWheelWait = 4;
      return true;
    }

    pub TextAreaClass2(
      tgame: GameClass,
      twidth: i32,
      trows: i32,
      tfont: Font,
      tText: String,
      let mut tItemSize: i32 = 16,
       tbackbitmap: Bitmap = null,
      let mut bbx: i32 = -1,
      let mut bby: i32 = -1,
      bool tWithoutScrollbars = false,
      bool tWithoutFrame = false,
      let mut tfontcol: i32 = 0,
      bool tcenterit = false,
      let mut colr: i32 = 0,
      let mut colg: i32 = 0,
      let mut colb: i32 = 0,
      let mut cola: i32 = 0,
      bool tshadow = true,
      bool tUseEncy = false,
      bool tminimalHeight = false,
      bool tUseMin40width = true,
      bool tDarkerFrame = false)
      : base(twidth, 30 + (trows + 1) * tItemSize)
    {
      self.ListObj = new ListClass[10];
      self.TabName = new string[10];
      self.TabRect = Rectangle::new[100];
      self.encyId = new int[2, 2, 2];
      if (tgame.Data.Product >= 7)
      {
        tText = tText.Replace("<br>", "\r\n");
        tText = tText.Replace("<BR>", "\r\n");
      }
      if (twidth == 0)
        return;
      if (!Information.IsNothing( tText) && tText.Length > 2)
      {
        while (tText.IndexOf("\r\n", tText.Length - 2) > 0)
        {
          tText = Strings.Left(tText, tText.Length - 2);
          if (tText.Length <= 2)
            break;
        }
      }
      self.minimalHeight = tminimalHeight;
      self.ItemSize = tItemSize;
      self.Width = twidth;
      self.useEncy = tUseEncy;
      self.Height = 30 + (trows + 1) * tItemSize;
      if (self.minimalHeight)
        self.Height = (trows + 1) * tItemSize;
      self.game = tgame;
      self.shadow = tshadow;
      self.darkerFrame = tDarkerFrame;
      self.centerit = tcenterit;
      self.WithoutFrame = tWithoutFrame;
      self.WithoutScrollbars = tWithoutScrollbars;
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
      }
      self.fontcol = tfontcol != 0 ? Color.Black : Color.White;
      if (cola > 0)
        self.fontcol = Color.FromArgb(cola, colr, colg, colb);
      self.fontColHigh = self.fontcol;
      self.bx = bbx;
      self.by = bby;
      SizeF sizeF = SizeF::new();
      strArray1: Vec<String> = new string[10];
      self.Tab = 0;
      self.TabCount = -1;
      self.TabName[0] = "";
      let mut num1: i32 = 1;
      if (Information.IsNothing( tText))
        tText = "";
      tText = tText.Replace("\t", " ");
      let mut num2: i32 = 0;
      let mut num3: i32 = 0;
      while (num1 == 1)
      {
        num1 = 0;
        let mut num4: i32 = Strings.InStr(tText, "[tab]");
        if (num4 > 0)
        {
          let mut num5: i32 = Strings.InStr(tText, "[/tab]");
          if (num5 > num4 & num5 > 0)
          {
            str1: String = Strings.Mid(tText, num4 + Strings.Len("[tab]"), num5 - (num4 + Strings.Len("[tab]")));
            let mut num6: i32 = Strings.InStr(str1, ",");
            if (num6 > 0)
            {
              self += 1.TabCount;
              self.TabName[self.TabCount] = Strings.Left(str1, num6 - 1);
              str2: String = Strings.Mid(str1, num6 + 1);
              strArray1[self.TabCount] = str2;
              if (str2.Length > num2)
                num2 = str2.Length;
              let mut startIndex: i32 = 0;
              let mut num7: i32 = 0;
              for (; str2.IndexOf("\r\n", startIndex) > 0; startIndex = str2.IndexOf("\r\n", startIndex) + 1)
                num7 += 1;
              if (num7 > num3)
                num3 = num7;
              tText = Strings.Left(tText, num4 - 1) + Strings.Mid(tText, num5 + Strings.Len("[/tab]"));
              num1 = 1;
            }
          }
        }
      }
      if (self.TabCount == -1)
      {
        self.TabCount = 0;
        strArray1[0] = tText;
        str: String = tText;
        if (str.Length > num2)
          num2 = str.Length;
        let mut startIndex1: i32 = 0;
        let mut num8: i32 = 0;
        for (; str.IndexOf('\n'.ToString(), startIndex1) > 0; startIndex1 = str.IndexOf('\n'.ToString(), startIndex1) + 1)
          num8 += 1;
        for (let mut startIndex2: i32 = 0; str.IndexOf("\r\n", startIndex2) > 0; startIndex2 = str.IndexOf("\r\n", startIndex2) + 1)
          num8 += 1;
        if (num8 > num3)
          num3 = num8;
      }
      if (self.useEncy)
      {
        self.encyId = (int[,,]) null;
        self.encyId = new int[self.TabCount + 1 + 1,  Math.Round( (1 + num3) + Math.Ceiling( num2 / 20.0)) + 1, 201];
      }
      else
        self.encyId = (int[,,]) null;
      int[] numArray1 = new int[500];
      self.tStringList = (StringListClass) null;
      if ( self.game.Data.RuleVar[951] > 0.0)
      {
        let mut stringListById: i32 = self.game.HandyFunctionsObj.GetStringListByID( Math.Round( self.game.Data.RuleVar[951]));
        if (stringListById > -1)
        {
          self.tStringList = self.game.Data.StringListObj[stringListById].Clone();
          self.tStringList.ID = -1;
        }
      }
      let mut tabCount: i32 = self.TabCount;
      for (let mut index1: i32 = 0; index1 <= tabCount; index1 += 1)
      {
        self.ListObj[index1] = ListClass::new();
        Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
        self.OwnFont = tfont != null ? tfont : Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel);
        let mut num9: i32 = 1;
        self.clickscroll = 0;
        tText = strArray1[index1];
        let mut num10: i32 = 0;
        if (self.game.Data.Product >= 7 && Strings.InStr(tText, "<<") > 0 & Strings.InStr(tText, ">>") > 0)
        {
          SimpleStringList simpleStringList = SimpleStringList::new();
          while (Strings.InStr(tText, "<<") > 0 & Strings.InStr(tText, ">>") > 0)
          {
            let mut Start: i32 = Strings.InStr(tText, "<<");
            let mut num11: i32 = Strings.InStr(tText, ">>");
            str: String = Strings.Mid(tText, Start, num11 - Start + 2);
            oldValue: String = str;
            strArray2: Vec<String> = str.Replace("<<", "").Replace(">>", "").Split(';');
            if (strArray2.Length >= 2)
            {
              self.tStringList.AddRowWithData(strArray2[0], strArray2[1], strArray2[2], 0.ToString(), "", "OVERRULE", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "");
              tText = tText.Replace(oldValue, strArray2[0]);
            }
          }
        }
        int[] numArray2 = self.game.HandyFunctionsObj.RecodeTextStringToEncyIdsNew(tText,  self.tStringList);
        if (self.game.Data.Product >= 7 && Strings.InStr(tText, "{", CompareMethod.Text) > 0)
        {
          str: String = "";
          int[] arySrc = new int[numArray2.GetUpperBound(0) + 1];
          bool flag = false;
          let mut index2: i32 = 0;
          let mut length: i32 = tText.Length;
          for (let mut Start: i32 = 1; Start <= length; Start += 1)
          {
            Left: String = Strings.Mid(tText, Start, 1);
            if (Operators.CompareString(Left, "{", false) == 0)
              flag = true;
            if (!flag)
            {
              index2 += 1;
              str += Left;
              arySrc[index2] = numArray2[Start];
            }
            if (Operators.CompareString(Left, "}", false) == 0)
              flag = false;
          }
          tText = str;
          numArray2 = (int[]) Utils.CopyArray((Array) arySrc, (Array) new int[index2 + 1]);
        }
        let mut num12: i32 = 40;
        if (self.WithoutScrollbars)
          num12 = 40;
        if (!tUseMin40width)
          num12 = 0;
        while (Strings.Len(tText) > 0)
        {
          let mut num13: i32 = 1;
          str3: String = "";
          bool flag = false;
          while (num13 == 1)
          {
            let mut num14: i32 = Strings.InStr(tText, "\r\n");
            let mut num15: i32 = Strings.InStr(tText, " ");
            if (num15 == 0)
              num15 = 9999999;
            num16: i32;
            if (num14 < num15 & num14 > 0)
            {
              let mut num17: i32 = num14;
              num13 = 0;
              num16 = 0;
              if (num17 != 1)
              {
                if ( graphics.MeasureString(str3 + Strings.Left(tText, num17 - 1), self.OwnFont).Width <=  (self.Width - num12))
                  str3 += Strings.Left(tText, num17 - 1);
                else if (self.game.Data.Product >= 6)
                {
                  if ( graphics.MeasureString(Strings.Left(tText, num17 - 1), self.OwnFont).Width >  (self.Width - num12))
                  {
                    if (Operators.CompareString(str3, "", false) == 0)
                      str3 += Strings.Left(tText, num17 - 1);
                  }
                  else
                    num16 = 1;
                }
              }
              if (num16 == 0)
              {
                if (num17 < Strings.Len(tText))
                {
                  tText = Strings.Mid(tText, num17 + 2);
                  flag = true;
                }
                else
                  tText = "";
              }
            }
            else
            {
              let mut Length: i32 = Strings.InStr(tText, " ");
              str4: String = Length <= 0 ? tText : Strings.Left(tText, Length);
              let mut num18: i32 = 0;
              num13 = 0;
              if ( graphics.MeasureString(str3 + str4, self.OwnFont).Width <=  (self.Width - num12))
              {
                num9 = 1;
                num18 = 1;
              }
              else if (num9 == 1)
              {
                num9 = 0;
              }
              else
              {
                num9 = 1;
                num18 = 1;
              }
              if (num18 == 1)
              {
                str3 += str4;
                if (Length > 0)
                {
                  if (Strings.Len(tText) >= Length + 1)
                  {
                    tText = Strings.Mid(tText, Length + 1);
                    num13 = 1;
                  }
                  else
                  {
                    tText = "";
                    num13 = 0;
                  }
                }
                else
                {
                  tText = "";
                  num13 = 0;
                }
              }
            }
            if (self.game.Data.Product >= 6 & num16 == 1)
              break;
          }
          self.ListObj[index1].add(str3, 0);
          if (self.useEncy)
          {
            let mut index3: i32 = 0;
            if (num10 == 0)
              num10 = 1;
            let mut num19: i32 = num10;
            let mut num20: i32 = num10 + str3.Length - 1;
            for (let mut index4: i32 = num19; index4 <= num20; index4 += 1)
            {
              index3 += 1;
              if (index3 <= self.encyId.GetUpperBound(2))
              {
                try
                {
                  self.encyId[index1, self.ListObj[index1].ListCount, index3] = numArray2[index4];
                }
                catch (Exception ex)
                {
                  ProjectData.SetProjectError(ex);
                  self.ListSize = self.encyId.GetUpperBound(2);
                  if (self.TabCount >= 0)
                    --self.ListSize;
                  self.ListSelect = -1;
                  self.TopItem = 0;
                  self.MouseOver = true;
                  ProjectData.ClearProjectError();
                  return;
                }
              }
            }
            num10 += str3.Length;
            if (flag)
              num10 += 2;
          }
        }
      }
      self.ListSize = trows;
      if (self.TabCount >= 0)
        --self.ListSize;
      self.ListSelect = -1;
      self.TopItem = 0;
      self.MouseOver = true;
    }

    pub fn HeightUsed() => (self.ListObj[self.Tab].ListCount + 1) -> i32 * self.ItemSize;

    pub fn WidthUsed() -> i32
    {
      SizeF sizeF = SizeF::new();
      return self.ListObj[0].ListCount < 0 ? 0 :  Math.Round( Graphics.FromImage((Image) self.OwnBitmap).MeasureString(self.ListObj[0].ListName[0], self.OwnFont).Width);
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      self.game.EditObj.TipColor = 0;
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          self.game.EditObj.TipColor = self.MouseData[index];
          break;
        }
      }
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      SimpleStringList simpleStringList = SimpleStringList::new();
      self.ClearMouse();
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      if (!Information.IsNothing( self.backbitmap))
      {
        Expression.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( Expression,  self.backbitmap, 0, 0);
        Expression.CompositingMode = CompositingMode.SourceOver;
      }
      if (!self.WithoutFrame)
      {
        if (self.darkerFrame)
        {
          c1: Color = Color.FromArgb( byte.MaxValue,  Math.Round( self.game.MarcCol1.R / 2.0),  Math.Round( self.game.MarcCol1.G / 2.0),  Math.Round( self.game.MarcCol1.B / 2.0));
          c2: Color = Color.FromArgb( byte.MaxValue,  Math.Round( self.game.MarcCol2.R / 2.0),  Math.Round( self.game.MarcCol2.G / 2.0),  Math.Round( self.game.MarcCol2.B / 2.0));
          DrawMod.DrawBlockGradient2( Expression, 0, self.ItemSize, self.Width - 10, self.Height - self.ItemSize, c1, c2);
        }
        else
          DrawMod.DrawBlockGradient2( Expression, 0, self.ItemSize, self.Width - 10, self.Height - self.ItemSize, self.game.MarcCol1, self.game.MarcCol2);
      }
      if (self.TabCount < 0)
      {
        bitmap: Bitmap;
        return bitmap;
      }
      let mut num1: i32 = 0;
      let mut tabCount1: i32 = self.TabCount;
      for (let mut index: i32 = 0; index <= tabCount1; index += 1)
      {
        self.TabName[index] = Strings.UCase(self.TabName[index]);
        SizeF sizeF2 = index >= self.TabCount ? Expression.MeasureString(self.TabName[index], DrawMod.TGame.MarcFont5) : Expression.MeasureString(self.TabName[index] + " | ", DrawMod.TGame.MarcFont5);
        num1 =  Math.Round( ( num1 + sizeF2.Width));
      }
      let mut x1: i32 =  Math.Round(Math.Max(0.0,  (self.Width - 10 - num1) / 2.0));
      let mut tabCount2: i32 = self.TabCount;
      SizeF sizeF3;
      for (let mut index: i32 = 0; index <= tabCount2; index += 1)
      {
        if (index < self.TabCount)
        {
          if (self.Tab != index)
            DrawMod.DrawTextColouredMarc( Expression, self.TabName[index] + " | ", DrawMod.TGame.MarcFont5, x1 + 1, 0, Color.White);
          if (index == self.Tab)
          {
            DrawMod.DrawTextColouredMarc( Expression, self.TabName[index], DrawMod.TGame.MarcFont5, x1 + 1, 0, DrawMod.TGame.MarcCol5);
            sizeF3 = Expression.MeasureString(self.TabName[index], DrawMod.TGame.MarcFont5);
            DrawMod.DrawTextColouredMarc( Expression, " | ", DrawMod.TGame.MarcFont5,  Math.Round( (x1 + 1) +  sizeF3.Width - 4.0), 0, Color.White);
          }
          sizeF3 = Expression.MeasureString(self.TabName[index] + " | ", DrawMod.TGame.MarcFont5);
        }
        else
        {
          if (self.Tab != index | self.TabCount == 0)
            DrawMod.DrawTextColouredMarc( Expression, self.TabName[index], DrawMod.TGame.MarcFont5, x1 + 1, 0, Color.White);
          else if (index == self.Tab)
            DrawMod.DrawTextColouredMarc( Expression, self.TabName[index], DrawMod.TGame.MarcFont5, x1 + 1, 0, DrawMod.TGame.MarcCol5);
          sizeF3 = Expression.MeasureString(self.TabName[index], DrawMod.TGame.MarcFont5);
        }
        self.TabRect[index] = index >= self.TabCount ? Rectangle::new(x1, 0,  Math.Round( sizeF3.Width), self.ItemSize) : Rectangle::new(x1, 0,  Math.Round( sizeF3.Width -  Expression.MeasureString(" | ", DrawMod.TGame.MarcFont5).Width), self.ItemSize);
        x1 =  Math.Round( ( x1 + sizeF3.Width));
      }
      let mut num2: i32 = 0;
      let mut num3: i32 = 0;
      let mut num4: i32 = 0;
      if (self.minimalHeight)
        num4 = 30;
      if (self.TabCount > 0)
        num2 = -1;
      let mut topItem: i32 = self.TopItem;
      let mut num5: i32 = self.TopItem + self.ListSize;
      Rectangle rectangle;
      Rectangle trect;
      for (let mut index1: i32 = topItem; index1 <= num5; index1 += 1)
      {
        num3 += 1;
        if (index1 <= self.ListObj[self.Tab].ListCount)
        {
          str1: String = self.ListObj[self.Tab].ListName[index1];
          if (self.WithoutScrollbars && index1 == self.TopItem + self.ListSize + num2 & index1 != self.ListObj[self.Tab].ListCount & self.game.Data.Product < 6)
            str1 = Strings.Left(str1, Math.Max(0, Strings.Len(str1) - 3)) + "...";
          sizeF3 = Expression.MeasureString(self.ListObj[self.Tab].ListName[index1], self.OwnFont);
          let mut x2: i32 = 15;
          if (self.minimalHeight)
            x2 = 0;
          if (self.centerit)
            x2 =  Math.Round( self.Width / 2.0 -  sizeF3.Width / 2.0);
          int[] ints = new int[200];
          try
          {
            if (self.useEncy)
            {
              let mut index2: i32 = 1;
              do
              {
                ints[index2] = self.encyId[self.Tab, index1, index2];
                index2 += 1;
              }
              while (index2 <= 199);
            }
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            ProjectData.ClearProjectError();
          }
          if (self.useEncy & self.game.HandyFunctionsObj.EncyTextStringContainsId(ints))
          {
            let mut num6: i32 = 0;
            str2: String = "";
            let mut num7: i32 = -1;
            while (str1.Length > 0)
            {
              bool flag = true;
              str3: String = "";
              let mut num8: i32 = num7;
              while (flag)
              {
                if (ints[1] != num7 & num7 != -1)
                {
                  flag = false;
                  if (str3.Length <= 0)
                    num7 = ints[1];
                }
                else
                {
                  if (ints[1] != num7 & num7 == -1)
                    num7 = ints[1];
                  str3 += str1.Substring(0, 1);
                  str1 = str1.Substring(1);
                  let mut index3: i32 = 1;
                  do
                  {
                    ints[index3 - 1] = ints[index3];
                    index3 += 1;
                  }
                  while (index3 <= 199);
                }
                if (str1.Length < 1)
                  flag = false;
              }
              str2 += str3;
              let mut tdata: i32 = -1;
              if ((str1.Length == 0 | self.game.Data.Product >= 7 | Strings.InStr(str3, "[?") > 0) & num8 == -1 & num7 > -1)
                num8 = num7;
              if (!Information.IsNothing( self.tStringList) & num8 > 0 && self.tStringList.Width >= 3)
                tdata =  Math.Round(Conversion.Val(self.tStringList.Data[num8 - 1, 3]));
              num9: i32;
              num10: i32;
              num11: i32;
              if (Strings.InStr(str3, "[?") > 0)
              {
                num9 = 32;
                num10 = 38;
                num11 = 8;
              }
              else
              {
                sizeF3 = Expression.MeasureString(str3, self.OwnFont, 9999, StringFormat.GenericDefault);
                num9 =  Math.Round( sizeF3.Width);
                sizeF3 = Expression.MeasureString("x" + str3 + "x", self.OwnFont, 9999, StringFormat.GenericDefault);
                num10 =  Math.Round( sizeF3.Width);
                sizeF3 = Expression.MeasureString("xx", self.OwnFont, 9999, StringFormat.GenericDefault);
                num11 =  Math.Round( sizeF3.Width);
              }
              if (num8 > 0)
              {
                if (Strings.InStr(str3, "[?") > 0)
                {
                  if (self.game.Data.Product >= 7)
                  {
                    if (tdata > 0)
                      DrawMod.DrawBlock( Expression, x2 + num6 + 2, self.ItemSize * num3 + 15 - 0 - num4, num10 - num11 - 4, Math.Max(16, self.ItemSize) - 6,  byte.MaxValue, 170, 50, 148);
                    else
                      DrawMod.DrawBlock( Expression, x2 + num6 + 2, self.ItemSize * num3 + 15 - 0 - num4, num10 - num11 - 4, Math.Max(16, self.ItemSize) - 6,  byte.MaxValue,  byte.MaxValue, 0, 64);
                    DrawMod.DrawRectangle( Expression, x2 + num6 + 2, self.ItemSize * num3 + 15 - 0 - num4, num10 - num11 - 4, Math.Max(16, self.ItemSize) - 6, 0, 0, 0, 224);
                  }
                  else
                  {
                    if (tdata > 0)
                      DrawMod.DrawBlock( Expression, x2 + num6 + 2, self.ItemSize * num3 + 13 + 1 - num4, num10 - num11 - 4, Math.Max(16, self.ItemSize) - 4,  byte.MaxValue, 170, 50, 148);
                    else
                      DrawMod.DrawBlock( Expression, x2 + num6 + 2, self.ItemSize * num3 + 13 + 1 - num4, num10 - num11 - 4, Math.Max(16, self.ItemSize) - 4,  byte.MaxValue,  byte.MaxValue, 0, 64);
                    DrawMod.DrawRectangle( Expression, x2 + num6 + 2, self.ItemSize * num3 + 13 + 1 - num4, num10 - num11 - 4, Math.Max(16, self.ItemSize) - 4, 0, 0, 0, 224);
                  }
                  let mut num12: i32 =  Math.Round(0.0 + ( num9 / 2.0 -  Expression.MeasureString("?", self.OwnFont, 9999, StringFormat.GenericDefault).Width / 2.0));
                  tstring: String = "?";
                  if (!self.shadow)
                    DrawMod.DrawTextColouredNicely( Expression, tstring, self.OwnFont, num12 + x2 + num6, self.ItemSize * num3 + 13 - num4, self.fontColHigh);
                  else
                    DrawMod.DrawTextColouredMarc( Expression, tstring, self.OwnFont, num12 + x2 + num6, self.ItemSize * num3 + 13 - num4, self.fontColHigh);
                }
                else
                {
                  str4: String = Strings.Left(str3, 1).ToUpper() + Strings.Mid(str3, 2);
                  if (self.game.Data.Product >= 7)
                  {
                    if (simpleStringList.FindNr(str4) == -1)
                    {
                      DrawMod.DrawBlock( Expression, x2 + num6 + 2, self.ItemSize * num3 + 13 - num4 +  Math.Round( self.OwnFont.Height * 0.8), num10 - num11 + 1, 2, 55, 155, 155, 128);
                      simpleStringList.Add(str4, 0);
                    }
                  }
                  else
                    DrawMod.DrawBlock( Expression, x2 + num6 + 2, self.ItemSize * num3 + 13 - num4 +  Math.Round( self.OwnFont.Height * 0.8), num10 - num11 + 1, 2, 55, 155, 155, 128);
                  if (!self.shadow)
                    DrawMod.DrawTextColouredNicely( Expression, str4, self.OwnFont, x2 + num6, self.ItemSize * num3 + 13 - num4, self.fontColHigh);
                  else
                    DrawMod.DrawTextColouredMarc( Expression, str4, self.OwnFont, x2 + num6, self.ItemSize * num3 + 13 - num4, self.fontColHigh);
                }
              }
              else if (!self.shadow)
                DrawMod.DrawTextColouredNicely( Expression, str3, self.OwnFont, x2 + num6, self.ItemSize * num3 + 13 - num4, self.fontcol);
              else
                DrawMod.DrawTextColouredMarc( Expression, str3, self.OwnFont, x2 + num6, self.ItemSize * num3 + 13 - num4, self.fontcol);
              if (num8 > 0)
              {
                ttitle: String = self.tStringList.Data[num8 - 1, 1];
                ttext: String = self.tStringList.Data[num8 - 1, 2];
                if (self.tStringList.Width >= 5 & self.game.Data.Product >= 6)
                {
                  currentName: String = self.tStringList.Data[num8 - 1, 0];
                  str5: String = self.tStringList.Data[num8 - 1, 4];
                  Left: String = self.tStringList.Data[num8 - 1, 5];
                  if (str5.Length > 0)
                    ttext = ttext + "\r\n\r\n" + str5;
                  if (Left.Length > 0 & Operators.CompareString(Left, "OVERRULE", false) != 0)
                    ttext = ttext + "\r\n\r\n" + Left;
                  ttext = self.game.HandyFunctionsObj.CustomMouseOverLookups(ttext.Replace("<br>", "\r\n"), currentName);
                }
                rectangle = Rectangle::new(x2 + num6, self.ItemSize * num3 + 13 - num4, num10 - num11, self.ItemSize);
                trect = rectangle;
                self.AddMouse( trect, ttitle, ttext, tdata);
                num6 += num10 - num11;
              }
              else
                num6 += num9;
            }
          }
          else if (!self.shadow)
            DrawMod.DrawTextColouredNicely( Expression, str1, self.OwnFont, x2, self.ItemSize * num3 + 13 - num4, self.fontcol);
          else
            DrawMod.DrawTextColouredMarc( Expression, str1, self.OwnFont, x2, self.ItemSize * num3 + 13 - num4, self.fontcol);
        }
      }
      if (!self.WithoutFrame)
        DrawMod.DrawFrame( self.OwnBitmap,  self.backbitmap,  Expression, 0, self.ItemSize, self.Width - 10, self.Height - self.ItemSize, -1, -1);
      if (!self.WithoutScrollbars && self.ListSize < self.ListObj[self.Tab].ListCount)
      {
        let mut num13: i32 = self.Height - (self.ItemSize + 50 + 10);
        float num14 = self.ListObj[self.Tab].ListCount <= 0 ? 1f :  self.TopItem /  (self.ListObj[self.Tab].ListCount - self.ListSize);
        if ( num14 > 1.0)
          num14 = 1f;
        let mut num15: i32 =  Math.Round( Conversion.Int( num13 * num14));
        let mut x3: i32 = self.Width - 20;
        let mut num16: i32 = self.ItemSize + 25 + num15;
         let mut local1: &Graphics = &Expression;
        bitmap: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
         let mut local2: &Bitmap = &bitmap;
        trect = Rectangle::new(0, 3, 20, 4);
        let mut srcrect1: &Rectangle = &trect
        rectangle = Rectangle::new(x3, 28 + self.ItemSize, 20, self.Height - (56 + self.ItemSize));
        let mut destrect1: &Rectangle = &rectangle
        DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
         let mut local3: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
         let mut local4: &Bitmap = &bitmap;
        trect = Rectangle::new(0, 0, 20, 3);
        let mut srcrect2: &Rectangle = &trect
        rectangle = Rectangle::new(x3, self.ItemSize + 25, 20, 3);
        let mut destrect2: &Rectangle = &rectangle
        DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
         let mut local5: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
         let mut local6: &Bitmap = &bitmap;
        trect = Rectangle::new(0, 7, 20, 3);
        let mut srcrect3: &Rectangle = &trect
        rectangle = Rectangle::new(x3, self.Height - 28, 20, 3);
        let mut destrect3: &Rectangle = &rectangle
        DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
         let mut local7: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTUP);
         let mut local8: &Bitmap = &bitmap;
        let mut x4: i32 = x3;
        let mut y1: i32 = self.ItemSize + 16;
        DrawMod.DrawSimple( local7,  local8, x4, y1);
         let mut local9: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTDOWN);
         let mut local10: &Bitmap = &bitmap;
        let mut x5: i32 = x3;
        let mut y2: i32 = self.Height - 25;
        DrawMod.DrawSimple( local9,  local10, x5, y2);
         let mut local11: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
         let mut local12: &Bitmap = &bitmap;
        let mut x6: i32 = x3;
        let mut y3: i32 = num16;
        DrawMod.DrawSimple( local11,  local12, x6, y3);
      }
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return self.OwnBitmap;
    }

    pub fn ShiftLeft()
    {
      --self.Tab;
      self.TopItem = 0;
      if (self.Tab > self.TabCount)
        self.Tab = self.TabCount;
      if (0 <= self.Tab)
        return;
      self.Tab = 0;
    }

    pub fn ShiftRight()
    {
      self += 1.Tab;
      self.TopItem = 0;
      if (self.Tab > self.TabCount)
        self.Tab = self.TabCount;
      if (0 <= self.Tab)
        return;
      self.Tab = 0;
    }

    pub fn ShiftDown()
    {
      self += 1.TopItem;
      if (self.TopItem > self.ListObj[self.Tab].ListCount - self.ListSize)
        self.TopItem = self.ListObj[self.Tab].ListCount - self.ListSize;
      if (0 <= self.TopItem)
        return;
      self.TopItem = 0;
    }

    pub fn ShiftUp()
    {
      --self.TopItem;
      if (self.TopItem > self.ListObj[self.Tab].ListCount - self.ListSize)
        self.TopItem = self.ListObj[self.Tab].ListCount - self.ListSize;
      if (0 <= self.TopItem)
        return;
      self.TopItem = 0;
    }

    pub fn HandleMouseUp(x: i32, y: i32) -> i32
    {
      if (!(self.clickscroll == 1 | self.Scroller))
        return -1;
      self.clickscroll = 0;
      self.Scroller = false;
      return 1;
    }

    pub fn Click(x: i32, y: i32, let mut b: i32 = 1) -> i32
    {
      let mut num1: i32 = y;
      if (self.TabCount > 0 & num1 < self.ItemSize)
      {
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
      else if (x > self.Width - 20)
      {
        if (y >= self.ItemSize + 16 & y <= self.ItemSize + 16 + 10)
        {
          --self.TopItem;
          self.clickscroll = 0;
          if (0 > self.TopItem)
            self.TopItem = 0;
          return -1;
        }
        if (y > self.Height - 25 & y < self.Height - 25 + 10)
        {
          self += 1.TopItem;
          self.clickscroll = 0;
          if (self.TopItem > self.ListObj[self.Tab].ListCount - self.ListSize)
            self.TopItem = self.ListObj[self.Tab].ListCount - self.ListSize;
          if (0 > self.TopItem)
            self.TopItem = 0;
          return -1;
        }
        self.clickscroll = 1;
        self.Scroller = true;
        let mut num2: i32 = self.Height - (self.ItemSize + 50);
        let mut num3: i32 = num1 - (self.ItemSize + 25);
        if (num3 < 1)
          num3 = 1;
        self.TopItem =  Math.Round( ( num3 /  num2 *  (self.ListObj[self.Tab].ListCount - (self.ListSize - 1))));
        if (self.TopItem > self.ListObj[self.Tab].ListCount - self.ListSize)
          self.TopItem = self.ListObj[self.Tab].ListCount - self.ListSize;
        if (0 > self.TopItem)
          self.TopItem = 0;
        return -1;
      }
      num4: i32;
      return num4;
    }

    pub bool MouseMove(x: i32, y: i32)
    {
      let mut num1: i32 = y;
      if (self.clickscroll != 1)
        return false;
      self.clickscroll = 1;
      self.Scroller = true;
      let mut num2: i32 = self.Height - (self.ItemSize + 50);
      let mut num3: i32 = num1 - (self.ItemSize + 25);
      if (num3 < 1)
        num3 = 1;
      self.TopItem =  Math.Round( ( num3 /  num2 *  (self.ListObj[self.Tab].ListCount - (self.ListSize - 1))));
      if (self.TopItem > self.ListObj[self.Tab].ListCount - self.ListSize)
        self.TopItem = self.ListObj[self.Tab].ListCount - self.ListSize;
      if (0 > self.TopItem)
        self.TopItem = 0;
      return true;
    }
  }
}
