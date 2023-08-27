// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SpecialWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SpecialWindowClass2 : WindowClass
  {
     okId: i32;
     useWidth: i32;
     useHeight: i32;

    pub SpecialWindowClass2( tGame: GameClass, tUseWidth: i32, tUseHeight: i32)
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
      let mut id1: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      libName: String = "SE_Data";
      self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 210, 0, 0));
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 190, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 187, 0, 0));
      let mut stringListById3: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 169, 0, 0));
      let mut stringListById4: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 168, 0, 0));
      if (self.okId > 0)
      {
        self.RemoveSubPart(self.okId);
        self.okId = 0;
      }
      self.ClearMouse();
      self.NewBackGroundAndClearAll(self.useWidth, self.useHeight, -1);
      Graphics g1 = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawRepeatingBackground(g1, DrawMod.TGame.BACKGROUND3MARC, 0, 0, self.useWidth, self.useHeight);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      SimpleList simpleList1 = SimpleList::new();
      simpleList1.Add(201, 0, 202, CheckExistence: false);
      simpleList1.Add(202, 0, 203, CheckExistence: false);
      simpleList1.Add(202, 0, 204, CheckExistence: false);
      simpleList1.Add(203, 0, 205, CheckExistence: false);
      simpleList1.Add(203, 0, 207, CheckExistence: false);
      simpleList1.Add(203, 0, 220, CheckExistence: false);
      simpleList1.Add(204, 0, 220, CheckExistence: false);
      simpleList1.Add(204, 0, 206, CheckExistence: false);
      simpleList1.Add(204, 0, 208, CheckExistence: false);
      simpleList1.Add(204, 0, 209, CheckExistence: false);
      simpleList1.Add(207, 0, 210, CheckExistence: false);
      simpleList1.Add(207, 0, 211, CheckExistence: false);
      simpleList1.Add(208, 0, 211, CheckExistence: false);
      simpleList1.Add(208, 0, 212, CheckExistence: false);
      simpleList1.Add(209, 0, 213, CheckExistence: false);
      simpleList1.Add(209, 0, 214, CheckExistence: false);
      simpleList1.Add(210, 0, 215, CheckExistence: false);
      simpleList1.Add(211, 0, 215, CheckExistence: false);
      simpleList1.Add(211, 0, 216, CheckExistence: false);
      simpleList1.Add(212, 0, 217, CheckExistence: false);
      simpleList1.Add(213, 0, 218, CheckExistence: false);
      simpleList1.Add(209, 0, 214, CheckExistence: false);
      simpleList1.Add(217, 0, 219, CheckExistence: false);
      int[] numArray1 = new int[21];
      int[] numArray2 = new int[21];
      int[] numArray3 = new int[21];
      let mut index1: i32 = 1;
      do
      {
        num1: i32;
        num2: i32;
        num3: i32;
        if (index1 == 1)
        {
          num1 = 201;
          num2 = 2;
          num3 = 1;
        }
        if (index1 == 2)
        {
          num1 = 202;
          num2 = 3;
          num3 = 1;
        }
        if (index1 == 3)
        {
          num1 = 205;
          num2 = 1;
          num3 = 2;
        }
        if (index1 == 4)
        {
          num1 = 203;
          num2 = 2;
          num3 = 2;
        }
        if (index1 == 5)
        {
          num1 = 204;
          num2 = 3;
          num3 = 2;
        }
        if (index1 == 6)
        {
          num1 = 206;
          num2 = 4;
          num3 = 2;
        }
        if (index1 == 7)
        {
          num1 = 207;
          num2 = 1;
          num3 = 3;
        }
        if (index1 == 8)
        {
          num1 = 220;
          num2 = 2;
          num3 = 3;
        }
        if (index1 == 9)
        {
          num1 = 208;
          num2 = 3;
          num3 = 3;
        }
        if (index1 == 10)
        {
          num1 = 209;
          num2 = 4;
          num3 = 3;
        }
        if (index1 == 11)
        {
          num1 = 210;
          num2 = 1;
          num3 = 4;
        }
        if (index1 == 12)
        {
          num1 = 211;
          num2 = 2;
          num3 = 4;
        }
        if (index1 == 13)
        {
          num1 = 212;
          num2 = 3;
          num3 = 4;
        }
        if (index1 == 14)
        {
          num1 = 213;
          num2 = 4;
          num3 = 4;
        }
        if (index1 == 15)
        {
          num1 = 214;
          num2 = 5;
          num3 = 4;
        }
        if (index1 == 16)
        {
          num1 = 215;
          num2 = 1;
          num3 = 5;
        }
        if (index1 == 17)
        {
          num1 = 216;
          num2 = 2;
          num3 = 5;
        }
        if (index1 == 18)
        {
          num1 = 217;
          num2 = 3;
          num3 = 5;
        }
        if (index1 == 19)
        {
          num1 = 218;
          num2 = 4;
          num3 = 5;
        }
        if (index1 == 20)
        {
          num1 = 219;
          num2 = 3;
          num3 = 6;
        }
        numArray3[index1] = num1;
        numArray1[index1] = num2;
        numArray2[index1] = num3;
        index1 += 1;
      }
      while (index1 <= 20);
      let mut counter1: i32 = simpleList1.Counter;
      for (let mut index2: i32 = 0; index2 <= counter1; index2 += 1)
      {
        let mut num4: i32 = simpleList1.Id[index2];
        let mut num5: i32 = simpleList1.Data1[index2];
        let mut index3: i32 = -1;
        let mut index4: i32 = -1;
        let mut index5: i32 = 1;
        do
        {
          if (num4 == numArray3[index5])
            index3 = index5;
          if (num5 == numArray3[index5])
            index4 = index5;
          index5 += 1;
        }
        while (index5 <= 20);
        Rectangle groupRect1 = self.GetGroupRect(numArray1[index3], numArray2[index3]);
        Rectangle groupRect2 = self.GetGroupRect(numArray1[index4], numArray2[index4]);
        if (groupRect1.Y < groupRect2.Y)
        {
          DrawMod.drawLine( g1, groupRect1.X +  Math.Round( groupRect1.Width / 2.0), groupRect1.Y + groupRect1.Height, groupRect2.X +  Math.Round( groupRect2.Width / 2.0), groupRect2.Y, 108, 108, 108, 180, 3);
          DrawMod.drawLine( g1, groupRect1.X +  Math.Round( groupRect1.Width / 2.0), groupRect1.Y + groupRect1.Height, groupRect2.X +  Math.Round( groupRect2.Width / 2.0), groupRect2.Y, 208, 228, 228, 180);
        }
        if (groupRect1.X < groupRect2.X & groupRect1.Y == groupRect2.Y)
        {
          DrawMod.drawLine( g1, groupRect1.X + groupRect1.Width, groupRect1.Y +  Math.Round( groupRect1.Height / 2.0), groupRect2.X, groupRect2.Y +  Math.Round( groupRect2.Height / 2.0), 108, 108, 108, 180, 3);
          DrawMod.drawLine( g1, groupRect1.X + groupRect1.Width, groupRect1.Y +  Math.Round( groupRect1.Height / 2.0), groupRect2.X, groupRect2.Y +  Math.Round( groupRect2.Height / 2.0), 208, 228, 228, 180);
        }
        if (groupRect1.X > groupRect2.X & groupRect1.Y == groupRect2.Y)
        {
          DrawMod.drawLine( g1, groupRect1.X, groupRect1.Y +  Math.Round( groupRect1.Height / 2.0), groupRect2.X + groupRect2.Width, groupRect2.Y +  Math.Round( groupRect2.Height / 2.0), 108, 108, 108, 180, 3);
          DrawMod.drawLine( g1, groupRect1.X, groupRect1.Y +  Math.Round( groupRect1.Height / 2.0), groupRect2.X + groupRect2.Width, groupRect2.Y +  Math.Round( groupRect2.Height / 2.0), 208, 228, 228, 180);
        }
      }
      let mut index6: i32 = 1;
      do
      {
        let mut num6: i32 = numArray3[index6];
        Rectangle groupRect = self.GetGroupRect(numArray1[index6], numArray2[index6]);
        if (self.game.EventRelatedObj.Helper_AirEnabled())
        {
          if (num6 == 206)
            groupRect.Width *= 2;
          if (num6 == 202)
            groupRect.Width =  Math.Round( groupRect.Width * 1.5);
        }
        else if (num6 == 206)
          groupRect.Width =  Math.Round( groupRect.Width * 1.5);
        data1: String = self.game.Data.StringListObj[stringListById1].GetData(0, num6, 1);
        bool flag1 = false;
        bool flag2 = false;
        let mut index7: i32 =  Math.Round(Conversion.Val( self.game.Data.StringListObj[stringListById2].FindRow2(0, num6, 1, id1)));
        if (index7 > -1)
        {
          let mut num7: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index7, 2]));
          if (num7 >= 2)
            flag1 = true;
          else if (num7 >= 1)
            flag2 = true;
        }
        str1: String = "";
        ttext1: String;
        c1: Color;
        if (flag2)
        {
          ttext1 = "This Tech Group is open to discoveries. Once you have fully researched 3 Techs from this group you'll open up all the Tech Groups connected to " + data1 + ".";
          c1 = Color.LightGray;
          DrawMod.DrawBlockGradient2( g1, groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height, Color.FromArgb(100, 100, 120, 100), Color.FromArgb(50, 50, 60, 50));
        }
        else if (flag1)
        {
          ttext1 = "You have fully researched 3 or more Techs from this group. You have fully mastered it and have access to all connected Tech Groups.";
          c1 = Color.White;
          DrawMod.DrawBlockGradient2( g1, groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height, Color.FromArgb(100, 0, 150, 0), Color.FromArgb(50, 0, 75, 0));
        }
        else
        {
          ttext1 = "This Tech Group is still out of reach for making discoveries. You need to fully research 3 or more Techs in a Tech Group connecting to " + data1 + ".";
          c1 = Color.Gray;
          DrawMod.DrawBlockGradient2( g1, groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height, self.game.MarcCol1, self.game.MarcCol2);
        }
        let mut num8: i32 =  Math.Round( (groupRect.Height - 33) / 5.0);
        let mut num9: i32 = 28;
        Rectangle trect1;
        Rectangle trect2;
        if (num8 < 16)
        {
          DrawMod.DrawTextColouredMarcCenter( g1, data1, self.game.MarcFont5, groupRect.X +  Math.Round( groupRect.Width / 2.0), groupRect.Y + 3, c1);
          num9 = 18;
          trect1 = Rectangle::new(groupRect.X, groupRect.Y, groupRect.Width, 18);
          trect2 = trect1;
          self.AddMouse( trect2, data1, ttext1);
        }
        else
        {
          DrawMod.DrawTextColouredMarcCenter( g1, data1, self.game.MarcFont4, groupRect.X +  Math.Round( groupRect.Width / 2.0), groupRect.Y + 5, c1);
          trect2 = Rectangle::new(groupRect.X, groupRect.Y, groupRect.Width, 28);
          trect1 = trect2;
          self.AddMouse( trect1, data1, ttext1);
        }
        DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g1, groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height, -1, -1);
        SimpleList simpleList2 = SimpleList::new();
        let mut length: i32 = self.game.Data.StringListObj[stringListById1].Length;
        for (let mut index8: i32 = 0; index8 <= length; index8 += 1)
        {
          let mut num10: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[index8, 10]));
          if (num10 == num6)
          {
            let mut tid: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[index8, 0]));
            if (tid != num10)
              simpleList2.Add(tid, 1);
          }
        }
        let mut num11: i32 = groupRect.X + 10;
        let mut num12: i32 = groupRect.Y + num9;
        let mut width: i32 =  Math.Round( groupRect.Width / 2.0 - 20.0);
        if (self.game.EventRelatedObj.Helper_AirEnabled())
        {
          if (num6 == 206)
            width =  Math.Round( groupRect.Width / 4.0 - 20.0);
          if (num6 == 202)
            width =  Math.Round( groupRect.Width / 3.0 - 20.0);
        }
        else if (num6 == 206)
          width =  Math.Round( groupRect.Width / 3.0 - 20.0);
        let mut num13: i32 = num11 + width + 20;
        let mut num14: i32 = num12;
        let mut num15: i32 = num13 + width + 20;
        let mut num16: i32 = num12;
        let mut num17: i32 = num15 + width + 20;
        let mut num18: i32 = num12;
        let mut height: i32 =  Math.Round( (groupRect.Height - (num9 + 5)) / 5.0);
        let mut num19: i32 = 5;
        if (num6 != 206 & !(self.game.EventRelatedObj.Helper_AirEnabled() & num6 == 202))
        {
          num19 =  Math.Round(Math.Floor( simpleList2.Counter / 2.0)) + 1;
          if (num19 < 5)
          {
            num12 +=  Math.Round( (height * (5 - num19)) / 2.0);
            num14 +=  Math.Round( (height * (5 - num19)) / 2.0);
          }
        }
        let mut counter2: i32 = simpleList2.Counter;
        for (let mut index9: i32 = 0; index9 <= counter2; index9 += 1)
        {
          let mut row: i32 = self.game.Data.StringListObj[stringListById1].FindRow(0, simpleList2.Id[index9]);
          str2: String = self.game.Data.StringListObj[stringListById1].Data[row, 1];
          x: i32;
          num20: i32;
          if (index9 < num19)
          {
            x = num11;
            num20 = num12;
            num12 += height;
          }
          else if (index9 < num19 * 2 | num6 != 206 & !(self.game.EventRelatedObj.Helper_AirEnabled() & num6 == 202))
          {
            x = num13;
            num20 = num14;
            num14 += height;
          }
          else if (index9 < num19 * 3 | num6 != 206)
          {
            x = num15;
            num20 = num16;
            num16 += height;
          }
          else
          {
            x = num17;
            num20 = num18;
            num18 += height;
          }
          let mut num21: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[row, 2]));
          let mut index10: i32 =  Math.Round(Conversion.Val( self.game.Data.StringListObj[stringListById2].FindRow2(0, simpleList2.Id[index9], 1, id1)));
          let mut num22: i32 = 0;
          if (index10 > -1)
            num22 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index10, 2]));
          str3: String = self.game.Data.StringListObj[stringListById1].Data[row, 9];
          let mut num23: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[row, 3]));
          let mut num24: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].Data[row, 4]));
          str4: String = self.game.Data.StringListObj[stringListById1].Data[row, 7];
          str5: String = str4.ToLower();
          str6: String = "";
          let mut num25: i32 = 1;
          idValue: i32;
          do
          {
            if (Strings.InStr(str5, "tech.") > 0)
            {
              let mut Start: i32 = Strings.InStr(str5, "tech.");
              let mut num26: i32 = Strings.InStr(Start, str5, ">");
              idValue =  Math.Round(Conversion.Val(Strings.Mid(str5, Start + 5, num26 - (Start + 5))));
              if (idValue > 0)
              {
                data2: String = self.game.Data.StringListObj[stringListById1].GetData(0, idValue, 1);
                str6 = str6 + "• " + data2 + "\r\n";
              }
              str5 = str5.Replace("tech." + idValue.ToString(), "used." + idValue.ToString());
            }
            num25 += 1;
          }
          while (num25 <= 4);
          str1 = "";
          r: i32;
          g2: i32;
          b: i32;
          a: i32;
          c2: Color;
          str7: String;
          if (num22 >= 100)
          {
            r = 0;
            g2 = 150;
            b = 0;
            a = 128;
            c2 = Color.White;
            str7 = "You have fully researched " + str2 + ".";
          }
          else if (index10 > -1)
          {
            r = 110;
            g2 = 110;
            b = 0;
            a = 128;
            c2 = Color.Yellow;
            if (num22 > 0)
              str7 = "You have discovered " + str2 + ". But not fully researched. Research Level is at " + num22.ToString() + ".";
            else
              str7 = "You have discovered " + str2 + ". But research on it has not yet started.";
            if (num21 == 2)
              str7 += "\r\nKeep in mind this is a Linear research field and effects gradual. Arriving at level 100 will be extremely difficult.";
            if (num22 <= 0)
              ;
          }
          else if (!flag2 & !flag1)
          {
            r = 0;
            g2 = 0;
            b = 0;
            a = 128;
            c2 = Color.FromArgb( byte.MaxValue, 140, 140, 140);
            str7 = "You have not yet discovered " + str2 + ". Discovery is currently impossible.";
          }
          else
          {
            bool flag3 = true;
            if (str4.Length > 1)
            {
              eventRelatedObj: EventRelatedClass = self.game.EventRelatedObj;
              let mut id2: i32 = self.game.Data.StringListObj[stringListById4].ID;
              let mut id3: i32 = self.game.Data.StringListObj[stringListById3].ID;
              logicString: String = str4;
              Random random = (Random) null;
               Random local =  random;
              if (eventRelatedObj.CheckLogicStringStart(id2, id3, logicString, 0,  local) < 1)
                flag3 = false;
            }
            if (flag3)
            {
              r = 100;
              g2 = 100;
              b = 100;
              a = 128;
              c2 = Color.LightGray;
              str7 = "You have not yet discovered " + str2 + ". Discovery is possible.";
            }
            else
            {
              r = 0;
              g2 = 0;
              b = 0;
              a = 128;
              c2 = Color.FromArgb( byte.MaxValue, 140, 140, 140);
              str7 = "You have not yet discovered " + str2 + ". Discovery is currently impossible because of a 'requisite condition' missing.";
            }
          }
          tstring: String = "";
          color: Color;
          if (index10 == -1 | num22 == 0 & index10 > -1)
          {
            switch (num23)
            {
              case 1:
                str7 += "\r\n\r\nYou need the Economic Council to discover this Tech.";
                tstring = "Ec";
                color = Color.DarkBlue;
                break;
              case 2:
                str7 += "\r\n\r\nYou need the Military Research Council to discover this Tech.";
                tstring = "Mi";
                color = Color.DarkOliveGreen;
                break;
              case 33:
                str7 += "\r\n\r\nYou need the Applied Science Council to discover this Tech.";
                tstring = "Ap";
                color = Color.DarkSlateGray;
                break;
              case 61:
                str7 += "\r\n\r\nYou need the Military Research Council or Airforce Research Department to discover this Tech.";
                tstring = "Air";
                color = Color.FromArgb( byte.MaxValue, 140, 160, 180);
                break;
            }
          }
          str8: String = str2;
          let mut num27: i32 = 1;
          let mut num28: i32 = 0;
          idValue = 0;
          if (num22 > 0 & num22 < 100)
            idValue = 20;
          if (index10 == -1)
            idValue = 20;
          if (num22 == 0)
            idValue = 20;
          while (num27 > 0)
          {
            num27 = 0;
            sizeF = g1.MeasureString(str8, self.game.MarcFont5);
            if ( sizeF.Width >  (width - idValue))
            {
              num27 = 1;
              num28 = 1;
              str8 = Strings.Left(str8, str8.Length - 1);
            }
          }
          if (num28 == 1)
            str8 += ".";
          if (num22 > 0 & num22 < 100)
            str8 = str8 + " - " + num22.ToString();
          ttext2: String = str7 + "\r\n\r\n" + str3;
          if (str6.Length > 0)
            ttext2 = ttext2 + "\r\n\r\nThis Tech has the following prerequisites:\r\n" + str6;
          DrawMod.DrawBlock( g1, x - 2, num20 - 2, Math.Max(14, width - 4 + 5), Math.Max(14, height - 4), r, g2, b, a);
          DrawMod.DrawTextColouredMarc( g1, str8, self.game.MarcFont5, x, num20 - 1, c2);
          if (tstring.Length > 0)
          {
            DrawMod.DrawBlock( g1,  Math.Round( x +  sizeF.Width + 3.0), num20 - 1, 20, Math.Max(12, height - 6),  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredMarcCenter( g1, tstring, self.game.MarcFont5,  Math.Round( x +  sizeF.Width + 13.0), num20 - 1, c2);
          }
          trect2 = Rectangle::new(x - 4, num20 - 4, width, height);
          trect1 = trect2;
          self.AddMouse( trect1, str2 + " (#" + simpleList2.Id[index9].ToString() + ")", ttext2);
        }
        index6 += 1;
      }
      while (index6 <= 20);
      g1.Dispose();
    }

    pub Rectangle GetGroupRect(tgroupX: i32, tgroupY: i32)
    {
      let mut num1: i32 = 50;
      let mut num2: i32 = 104;
      let mut num3: i32 = self.useHeight - 768;
      let mut num4: i32 =  Math.Round(238.0 + Math.Floor( (self.useWidth - 1280) / 5.0));
      let mut num5: i32 =  Math.Round(108.0 + Math.Floor( num3 / 6.0));
      let mut x: i32 = (tgroupX - 1) * num4 + num1;
      let mut y: i32 = (tgroupY - 1) * num5 + num2;
      let mut num6: i32 = num4 - 8;
      let mut height: i32 =  Math.Round( (num5 - 18) - Math.Floor( num3 / 18.0));
      let mut width: i32 =  Math.Round( num6 - Math.Floor( num3 / 15.0));
      return Rectangle::new(x, y, width, height);
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
