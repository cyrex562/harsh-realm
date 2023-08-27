// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SpecialWindowClass1
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SpecialWindowClass1 : WindowClass
  {
     okId: i32;
     useWidth: i32;
     useHeight: i32;

    pub SpecialWindowClass1( tGame: GameClass, tUseWidth: i32, tUseHeight: i32)
      : base( tGame, tUseWidth, tUseHeight, 8)
    {
      self.useWidth = tUseWidth;
      self.useHeight = tUseHeight;
      self.dostuff();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            self.SubPartList[index].DescriptInfo(x - self.SubPartX[index], y - self.SubPartY[index]);
            if (Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
            {
              self.game.EditObj.TipButton = true;
              self.game.EditObj.TipTitle = "";
              self.game.EditObj.TipText = self.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 0)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          break;
        }
      }
    }

    pub fn dostuff(bool crmAlreadySet = false)
    {
      SizeF sizeF = SizeF::new();
      let mut id: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      libName: String = "SE_Data";
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 210, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 387, 0, 0));
      let mut stringListById3: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 388, 0, 0));
      let mut stringListById4: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 258, 0, 0));
      let mut stringListById5: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 277, 0, 0));
      let mut stringListById6: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 402, 0, 0));
      let mut stringListById7: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 188, 0, 0));
      let mut stringListById8: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Present", 81, 0, 0));
      if (self.okId > 0)
      {
        self.RemoveSubPart(self.okId);
        self.okId = 0;
      }
      self.ClearMouse();
      self.NewBackGroundAndClearAll(self.useWidth, self.useHeight, -1);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawRepeatingBackground(g, DrawMod.TGame.BACKGROUND3MARC, 0, 0, self.useWidth, self.useHeight);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      let mut num1: i32 = 50;
      let mut num2: i32 = 88;
      let mut num3: i32 =  Math.Round( (self.useHeight - 768) / 9.0) - 1;
      if (num3 < 0)
        num3 = 0;
      let mut num4: i32 =  Math.Round( (self.useWidth - 310) / 6.0);
      if (num4 < 160)
        num4 = 160;
      let mut Length: i32 =  Math.Round( num4 / 8.0) - 1;
      if (Length < 19)
        Length = 10;
      let mut num5: i32 = 1;
      do
      {
        let mut num6: i32 = num1;
        let mut num7: i32 = num2 + (num5 - 1) * (72 + num3);
        DrawMod.DrawBlockGradient2( g, num6, num7, self.useWidth - 100, 70 + num3, self.game.MarcCol1, self.game.MarcCol2);
        DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, num6, num7, self.useWidth - 100, 70 + num3, -1, -1);
        str1: String = "";
        if (num5 == 1)
          str1 = "Democracy";
        if (num5 == 2)
          str1 = "Autocracy";
        if (num5 == 3)
          str1 = "Meritocracy";
        if (num5 == 4)
          str1 = "Enforcement";
        if (num5 == 5)
          str1 = "Commerce";
        if (num5 == 6)
          str1 = "Government";
        if (num5 == 7)
          str1 = "Fist";
        if (num5 == 8)
          str1 = "Mind";
        if (num5 == 9)
          str1 = "Heart";
        let mut num8: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData2(0, id, 1, str1, 2)));
        data1: String = self.game.Data.StringListObj[stringListById4].GetData(0, str1, 4);
        let mut tCardId: i32 = self.game.Data.FindEventPic( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData(0, str1, 5))), data1);
        let mut num9: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById4].GetData(0, str1, 6)));
         let mut local1: &Graphics = &g;
        bitmap1: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[tCardId]);
         let mut local2: &Bitmap = &bitmap1;
        Rectangle trect = Rectangle::new(num9 * 32, 0, 32, 32);
        let mut srcrect: &Rectangle = &trect
        Rectangle rectangle = Rectangle::new(num6 + 20, num7 + 20 +  Math.Round( num3 / 2.0), 32, 32);
        let mut destrect: &Rectangle = &rectangle
        DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
        DrawMod.DrawTextColouredMarc( g, num8.ToString(), self.game.MarcFont3, num6 + 58, num7 + 25 +  Math.Round( num3 / 2.0), Color.White);
        DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, num6 + 88, num7 + 26 +  Math.Round( num3 / 2.0), Color.White);
        let mut num10: i32 = num7 + 7;
        let mut num11: i32 = 1;
        do
        {
          let mut num12: i32 = num1 + 200 + (num11 - 1) * num4;
          let mut idValue2: i32 = 40 + (num11 - 1) * 10;
          let mut num13: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData2(5, str1.ToLower(), 20, idValue2, 0)));
          if (num13 > 0)
          {
            bool flag1 = false;
            if (self.game.Data.StringListObj[stringListById3].FindRow2(0, id, 1, num13) > -1)
              flag1 = true;
            str2: String = self.game.Data.StringListObj[stringListById2].GetData(0, num13, 3);
            currentName: String = str2;
            let mut num14: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, num13, 21)));
            if (str2.Length > Length + 1)
              str2 = Strings.Left(str2, Length) + ".";
            bool flag2 = false;
            let mut num15: i32 = 0;
            if (num8 >= idValue2 & !flag1)
            {
              flag2 = true;
              num15 = num8 - idValue2 + 1;
            }
            bool flag3 = false;
            let mut num16: i32 = 0;
            if (flag1)
              flag2 = false;
            if (num8 <= num14)
            {
              flag3 = true;
              num16 = num14 - num8 + 1;
            }
            if (!flag1)
              flag3 = false;
            if (flag1)
            {
              if (flag3)
                DrawMod.DrawBlockGradient2( g, num12, num10, num4 - 5, 55 + num3, Color.FromArgb(100, 100, 70, 0), Color.FromArgb(50, 50, 30, 0));
              else
                DrawMod.DrawBlockGradient2( g, num12, num10, num4 - 5, 55 + num3, Color.FromArgb(100, 0, 150, 0), Color.FromArgb(50, 0, 75, 0));
            }
            else if (!flag1)
            {
              if (flag2)
                DrawMod.DrawBlockGradient2( g, num12, num10, num4 - 5, 55 + num3, Color.FromArgb(100, 100, 120, 100), Color.FromArgb(50, 50, 60, 50));
              else
                DrawMod.DrawBlockGradient2( g, num12, num10, num4 - 5, 55 + num3, Color.FromArgb(100, 0, 0, 0), Color.FromArgb(50, 0, 0, 0));
            }
            data2: String = self.game.Data.StringListObj[stringListById2].GetData(0, num13, 22);
            let mut num17: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, num13, 23)));
            bool flag4 = true;
            let mut num18: i32 = num17;
            let mut num19: i32 = 0;
            if (data2.Length > 1)
            {
              let mut num20: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData2(0, id, 1, data2, 2)));
              num19 = num20;
              if (num20 < num17)
                flag4 = false;
            }
            else
              flag4 = false;
            let mut num21: i32 = num12 + 8;
            let mut length1: i32 = self.game.Data.StringListObj[stringListById5].Length;
            str3: String;
            for (let mut index: i32 = 0; index <= length1; index += 1)
            {
              if (Strings.InStr(self.game.Data.StringListObj[stringListById5].Data[index, 6], "REGIMEFEAT." + num13.ToString()) > 0)
              {
                tCardId =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById5].Data[index, 0]));
                let mut num22: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById5].Data[index, 5]));
                let mut num23: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById5].Data[index, 14]));
                ttitle: String = "Stratagem: " + self.game.Data.StringListObj[stringListById5].Data[index, 1];
                str3 = "";
                str4: String;
                if (num22 > 0 | num23 > 0)
                {
                  if (num22 == 0 | num23 == 0)
                  {
                    let mut idValue: i32 = Math.Max(num22, num23);
                    str4 = "You need the Organisation '" + self.game.Data.StringListObj[stringListById7].GetData(0, idValue, 1) + "' to generate these Stratagems.\r\n\r\n";
                  }
                  else
                    str4 = "You need the Organisation '" + self.game.Data.StringListObj[stringListById7].GetData(0, num22, 1) + "' or '" + self.game.Data.StringListObj[stringListById7].GetData(0, num23, 1) + "' to generate these Stratagems.\r\n\r\n";
                }
                else
                  str4 = "No Organisation is needed to generate these Stratagems.\r\n\r\n";
                ttext: String = str4 + self.game.Data.StringListObj[stringListById5].Data[index, 4];
                let mut x1: i32 = num21;
                let mut y1: i32 =  Math.Round( (num10 + 19) +  num3 * 0.33);
                let mut width: i32 =  Math.Round(20.0 +  num3 * 0.44);
                let mut height: i32 =  Math.Round(30.0 +  num3 * 0.66);
                 let mut local3: &Graphics = &g;
                bitmap2: Bitmap = self.game.CustomBitmapObj.DrawActionCardSe1(self.game.Data.Turn, -1, size: 2, tCardId: tCardId);
                 let mut local4: &Bitmap = &bitmap2;
                let mut x2: i32 = x1;
                let mut y2: i32 = y1;
                let mut w: i32 = width;
                let mut h: i32 = height;
                DrawMod.DrawScaled( local3,  local4, x2, y2, w, h, true);
                rectangle = Rectangle::new(x1, y1, width, height);
                trect = rectangle;
                self.AddMouse( trect, ttitle, ttext);
                num21 += width + 2;
              }
            }
            let mut length2: i32 = self.game.Data.StringListObj[stringListById6].Length;
            for (let mut index: i32 = 0; index <= length2; index += 1)
            {
              if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById6].Data[index, 1])) == num13)
              {
                tCardId =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById6].Data[index, 10]));
                ttitle: String = "Unit Feat: " + self.game.Data.StringListObj[stringListById6].Data[index, 2];
                ttext: String = self.game.Data.StringListObj[stringListById6].Data[index, 12];
                bitmap3: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[tCardId]);
                let mut x: i32 = num21 - 4;
                let mut y: i32 =  Math.Round( (num10 + 10) +  num3 * 0.5);
                let mut num24: i32 =  Math.Round(40.0 +  num3 * 0.5);
                let mut num25: i32 =  Math.Round(40.0 +  num3 * 0.5);
                DrawMod.DrawScaled( g,  bitmap3, x, y, num24, num25, true);
                rectangle = Rectangle::new(x, y, num24, num25);
                trect = rectangle;
                self.AddMouse( trect, ttitle, ttext);
              }
            }
            if (flag1)
              DrawMod.DrawTextColouredMarc( g, str2, self.game.MarcFont8c, num12 + 8,  Math.Round( (num10 + 4) +  num3 / 8.0), Color.White);
            else if (!flag1)
              DrawMod.DrawTextColouredMarc( g, str2, self.game.MarcFont8c, num12 + 8,  Math.Round( (num10 + 4) +  num3 / 8.0), Color.LightGray);
            DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, num12, num10, num4 - 5, 55 + num3, -1, -1);
            tCardId = self.game.Data.StringListObj[stringListById8].FindRow(0, "{regimefeat}" + currentName);
            if (tCardId > -1)
            {
              str5: String = self.game.Data.StringListObj[stringListById8].Data[tCardId, 2] + "\r\n\r\n" + self.game.Data.StringListObj[stringListById8].Data[tCardId, 5];
              str3 = "";
              str6: String;
              if (flag1)
              {
                if (flag3)
                  str6 = "Your nation has this Regime Feat. You have a " + num16.ToString() + "% chance every turn to lose this Regime Feat because your profile is equal or below " + str1 + "-" + num14.ToString() + ".";
                else
                  str6 = "Your nation has this Regime Feat. You cannot lose it unless your Profile drops to " + str1 + "-" + num14.ToString() + " or lower.";
              }
              else if (flag2)
              {
                if (flag4)
                  str6 = "Your nation does not have this Regime Feat. You have a " + num15.ToString() + "% chance every turn to gain it. Has been the highest Profile in group for " + num19.ToString() + " Rounds.";
                else
                  str6 = "Your nation does not have this Regime Feat. You still have 0% chance every turn to gain it because the Profile has only been the highest Profile in group for " + num19.ToString() + " Rounds. It needs to have been the highest for " + num18.ToString() + " Rounds.";
              }
              else
                str6 = "Your nation does not have this Regime Feat. You need a Profile of at least " + str1 + "-" + idValue2.ToString() + " to have a chance to gain it.";
              ttext: String = self.game.HandyFunctionsObj.CustomMouseOverLookups(str6 + "\r\n\r\n" + str5, currentName);
              rectangle = Rectangle::new(num12, num10, num4 - 5, 55 + num3);
              trect = rectangle;
              self.AddMouse( trect, "Regime Feat: " + currentName, ttext);
            }
            if (flag2)
            {
              if (flag4)
              {
                DrawMod.DrawTextColouredMarcCenter( g, num15.ToString() + "%", self.game.MarcFont4, num12 + num4 - 24, num10 + 36 + num3, Color.FromArgb( byte.MaxValue, 150, 200, 150));
              }
              else
              {
                tCardId = num18 - num19;
                if (tCardId < 0)
                  tCardId = 0;
                DrawMod.DrawTextColouredMarcCenter( g, tCardId.ToString() + " Rd", self.game.MarcFont4, num12 + num4 - 36, num10 + 36 + num3, Color.FromArgb( byte.MaxValue, 200, 150, 150));
              }
            }
            if (flag3)
              DrawMod.DrawTextColouredMarcCenter( g, num16.ToString() + "%", self.game.MarcFont4, num12 + num4 - 24, num10 + 36 + num3, Color.FromArgb( byte.MaxValue, 200, 150, 150));
          }
          num11 += 1;
        }
        while (num11 <= 6);
        num5 += 1;
      }
      while (num5 <= 9);
      g.Dispose();
      g = (Graphics) null;
    }

    pub HandleKeyup: WindowReturnClass(nr: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          if (nr > 0 & self.okId > 0)
          {
            windowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.okId)] + 1, self.SubPartY[self.SubpartNr(self.okId)] + 1, 1);
            windowReturnClass.SetFlag(true);
          }
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass2: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index] && self.SubPartID[index] == self.okId)
          {
            windowReturnClass1.AddCommand(6, 0);
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
        }
        windowReturnClass1.SetFlag(false);
        return windowReturnClass1;
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }
  }
}
