// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SpecialWindowClass3
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SpecialWindowClass3 : WindowClass
  {
     okId: i32;
     useWidth: i32;
     useHeight: i32;

    pub SpecialWindowClass3( tGame: GameClass, tUseWidth: i32, tUseHeight: i32)
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
      SizeF sizeF1 = SizeF::new();
      let mut id1: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      libName: String = "SE_Data";
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 225, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 228, 0, 0));
      let mut stringListById3: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 169, 0, 0));
      let mut stringListById4: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 168, 0, 0));
      let mut stringListById5: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 143, 0, 0));
      let mut stringListById6: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 306, 0, 0));
      let mut stringListById7: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 190, 0, 0));
      if (self.okId > 0)
      {
        self.RemoveSubPart(self.okId);
        self.okId = 0;
      }
      let mut idValue1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById5].GetData(0, id1, 2)));
      let mut cultureGroupId: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById6].GetData(0, idValue1, 1)));
      self.game.Data.StringListObj[stringListById3].SetData(0, "REGIMEID", 1, id1);
      self.game.Data.StringListObj[stringListById3].SetData(0, "ROUND", 1, self.game.Data.Round);
      self.ClearMouse();
      self.NewBackGroundAndClearAll(self.useWidth, self.useHeight, -1);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawRepeatingBackground(g, DrawMod.TGame.BACKGROUND3MARC, 0, 0, self.useWidth, self.useHeight);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      text: String;
      sizeF1 = g.MeasureString(text, self.game.MarcFont4);
      SimpleList simpleList1 = SimpleList::new();
      simpleList1.Add(1, 0, 2, CheckExistence: false);
      simpleList1.Add(1, 0, 3, CheckExistence: false);
      simpleList1.Add(1, 0, 22, CheckExistence: false);
      simpleList1.Add(1, 0, 11, CheckExistence: false);
      simpleList1.Add(1, 0, 41, CheckExistence: false);
      simpleList1.Add(1, 0, 5, CheckExistence: false);
      simpleList1.Add(1, 0, 13, CheckExistence: false);
      simpleList1.Add(2, 0, 73, CheckExistence: false);
      simpleList1.Add(73, 0, 72, CheckExistence: false);
      simpleList1.Add(2, 0, 4, CheckExistence: false);
      simpleList1.Add(4, 0, 81, CheckExistence: false);
      simpleList1.Add(22, 0, 21, CheckExistence: false);
      simpleList1.Add(22, 0, 23, CheckExistence: false);
      simpleList1.Add(23, 0, 61, CheckExistence: false);
      simpleList1.Add(61, 0, 62, CheckExistence: false);
      simpleList1.Add(62, 0, 63, CheckExistence: false);
      simpleList1.Add(63, 0, 64, CheckExistence: false);
      simpleList1.Add(11, 0, 31, CheckExistence: false);
      simpleList1.Add(31, 0, 32, CheckExistence: false);
      simpleList1.Add(32, 0, 34, CheckExistence: false);
      simpleList1.Add(32, 0, 91, CheckExistence: false);
      simpleList1.Add(32, 0, 33, CheckExistence: false);
      simpleList1.Add(33, 0, 51, CheckExistence: false);
      simpleList1.Add(91, 0, 92, CheckExistence: false);
      simpleList1.Add(33, 0, 35, CheckExistence: false);
      simpleList1.Add(33, 0, 36, CheckExistence: false);
      simpleList1.Add(51, 0, 52, CheckExistence: false);
      simpleList1.Add(51, 0, 71, CheckExistence: false);
      simpleList1.Add(41, 0, 12, CheckExistence: false);
      if (self.game.EventRelatedObj.Helper_AirEnabled())
      {
        simpleList1.Add(1, 0, 101, CheckExistence: false);
        simpleList1.Add(1, 0, 111, CheckExistence: false);
        simpleList1.Add(1, 0, 121, CheckExistence: false);
        simpleList1.Add(101, 0, 102, CheckExistence: false);
        simpleList1.Add(102, 0, 103, CheckExistence: false);
        simpleList1.Add(103, 0, 105, CheckExistence: false);
        simpleList1.Add(105, 0, 107, CheckExistence: false);
        simpleList1.Add(103, 0, 104, CheckExistence: false);
        simpleList1.Add(105, 0, 106, CheckExistence: false);
        simpleList1.Add(107, 0, 108, CheckExistence: false);
        simpleList1.Add(111, 0, 112, CheckExistence: false);
        simpleList1.Add(112, 0, 113, CheckExistence: false);
        simpleList1.Add(121, 0, 122, CheckExistence: false);
        simpleList1.Add(122, 0, 123, CheckExistence: false);
        simpleList1.Add(21, 0, 132, CheckExistence: false);
        simpleList1.Add(62, 0, 131, CheckExistence: false);
        simpleList1.Add(3, 0, 133, CheckExistence: false);
      }
      int[] numArray1 = new int[56];
      int[] numArray2 = new int[56];
      int[] numArray3 = new int[56];
      let mut index1: i32 = 1;
      do
      {
        num1: i32;
        num2: i32;
        num3: i32;
        if (index1 == 1)
        {
          num1 = 1;
          num2 = 1;
          num3 = 1;
        }
        if (index1 == 2)
        {
          num1 = 2;
          num2 = 2;
          num3 = 1;
        }
        if (index1 == 3)
        {
          num1 = 73;
          num2 = 3;
          num3 = 1;
        }
        if (index1 == 4)
        {
          num1 = 72;
          num2 = 4;
          num3 = 1;
        }
        if (index1 == 5)
        {
          num1 = 3;
          num2 = 2;
          num3 = 2;
        }
        if (index1 == 6)
        {
          num1 = 4;
          num2 = 3;
          num3 = 2;
        }
        if (index1 == 7)
        {
          num1 = 81;
          num2 = 4;
          num3 = 2;
        }
        if (index1 == 8)
        {
          num1 = 22;
          num2 = 2;
          num3 = 3;
        }
        if (index1 == 9)
        {
          num1 = 21;
          num2 = 3;
          num3 = 3;
        }
        if (index1 == 10)
        {
          num1 = 23;
          num2 = 3;
          num3 = 4;
        }
        if (index1 == 11)
        {
          num1 = 61;
          num2 = 4;
          num3 = 4;
        }
        if (index1 == 12)
        {
          num1 = 62;
          num2 = 5;
          num3 = 4;
        }
        if (index1 == 13)
        {
          num1 = 63;
          num2 = 6;
          num3 = 4;
        }
        if (index1 == 14)
        {
          num1 = 64;
          num2 = 7;
          num3 = 4;
        }
        if (index1 == 15)
        {
          num1 = 34;
          num2 = 5;
          num3 = 5;
        }
        if (index1 == 16)
        {
          num1 = 11;
          num2 = 2;
          num3 = 6;
        }
        if (index1 == 17)
        {
          num1 = 31;
          num2 = 3;
          num3 = 6;
        }
        if (index1 == 18)
        {
          num1 = 32;
          num2 = 4;
          num3 = 6;
        }
        if (index1 == 19)
        {
          num1 = 91;
          num2 = 5;
          num3 = 6;
        }
        if (index1 == 20)
        {
          num1 = 92;
          num2 = 6;
          num3 = 6;
        }
        if (index1 == 21)
        {
          num1 = 33;
          num2 = 5;
          num3 = 7;
        }
        if (index1 == 22)
        {
          num1 = 35;
          num2 = 6;
          num3 = 7;
        }
        if (index1 == 23)
        {
          num1 = 41;
          num2 = 2;
          num3 = 8;
        }
        if (index1 == 24)
        {
          num1 = 12;
          num2 = 3;
          num3 = 8;
        }
        if (index1 == 25)
        {
          num1 = 36;
          num2 = 6;
          num3 = 8;
        }
        if (index1 == 26)
        {
          num1 = 5;
          num2 = 2;
          num3 = 9;
        }
        if (index1 == 27)
        {
          num1 = 51;
          num2 = 6;
          num3 = 9;
        }
        if (index1 == 28)
        {
          num1 = 52;
          num2 = 7;
          num3 = 9;
        }
        if (index1 == 29)
        {
          num1 = 13;
          num2 = 2;
          num3 = 10;
        }
        if (index1 == 30)
        {
          num1 = 71;
          num2 = 7;
          num3 = 10;
        }
        if (self.game.EventRelatedObj.Helper_AirEnabled())
        {
          if (index1 == 26)
          {
            num1 = 5;
            num2 = 2;
            num3 = 4;
          }
          if (index1 == 29)
          {
            num1 = 13;
            num2 = 2;
            num3 = 5;
          }
          if (index1 == 23)
          {
            num1 = 41;
            num2 = 2;
            num3 = 7;
          }
          if (index1 == 24)
          {
            num1 = 12;
            num2 = 3;
            num3 = 7;
          }
          if (index1 == 31)
          {
            num1 = 101;
            num2 = 2;
            num3 = 10;
          }
          if (index1 == 32)
          {
            num1 = 102;
            num2 = 3;
            num3 = 10;
          }
          if (index1 == 33)
          {
            num1 = 103;
            num2 = 4;
            num3 = 10;
          }
          if (index1 == 34)
          {
            num1 = 105;
            num2 = 5;
            num3 = 10;
          }
          if (index1 == 35)
          {
            num1 = 107;
            num2 = 6;
            num3 = 10;
          }
          if (index1 == 36)
          {
            num1 = 104;
            num2 = 4;
            num3 = 11;
          }
          if (index1 == 37)
          {
            num1 = 106;
            num2 = 5;
            num3 = 11;
          }
          if (index1 == 38)
          {
            num1 = 108;
            num2 = 6;
            num3 = 11;
          }
          if (index1 == 39)
          {
            num1 = 111;
            num2 = 2;
            num3 = 9;
          }
          if (index1 == 40)
          {
            num1 = 112;
            num2 = 3;
            num3 = 9;
          }
          if (index1 == 41)
          {
            num1 = 113;
            num2 = 4;
            num3 = 9;
          }
          if (index1 == 42)
          {
            num1 = 121;
            num2 = 2;
            num3 = 8;
          }
          if (index1 == 43)
          {
            num1 = 122;
            num2 = 3;
            num3 = 8;
          }
          if (index1 == 44)
          {
            num1 = 123;
            num2 = 4;
            num3 = 8;
          }
          if (index1 == 45)
          {
            num1 = 131;
            num2 = 6;
            num3 = 3;
          }
          if (index1 == 46)
          {
            num1 = 132;
            num2 = 4;
            num3 = 3;
          }
          if (index1 == 47)
          {
            num1 = 133;
            num2 = 3;
            num3 = 2;
          }
          if (index1 == 2)
          {
            num1 = 2;
            num2 = 3;
            num3 = 1;
          }
          if (index1 == 3)
          {
            num1 = 73;
            num2 = 4;
            num3 = 1;
          }
          if (index1 == 4)
          {
            num1 = 72;
            num2 = 5;
            num3 = 1;
          }
          if (index1 == 6)
          {
            num1 = 4;
            num2 = 4;
            num3 = 2;
          }
          if (index1 == 7)
          {
            num1 = 81;
            num2 = 5;
            num3 = 2;
          }
        }
        numArray3[index1] = num1;
        numArray1[index1] = num2;
        numArray2[index1] = num3;
        index1 += 1;
      }
      while (index1 <= 47);
      let mut counter: i32 = simpleList1.Counter;
      for (let mut index2: i32 = 0; index2 <= counter; index2 += 1)
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
        while (index5 <= 47);
        Rectangle groupRect1 = self.GetGroupRect(numArray1[index3], numArray2[index3]);
        Rectangle groupRect2 = self.GetGroupRect(numArray1[index4], numArray2[index4]);
        if (num4 == 1 & !(num5 == 2 | num5 == 3))
        {
          DrawMod.drawLine( g, groupRect1.X +  Math.Round( groupRect1.Width / 2.0), groupRect1.Y + groupRect1.Height, groupRect2.X, groupRect2.Y +  Math.Round( groupRect2.Height / 2.0), 108, 108, 108, 180, 3);
          DrawMod.drawLine( g, groupRect1.X +  Math.Round( groupRect1.Width / 2.0), groupRect1.Y + groupRect1.Height, groupRect2.X, groupRect2.Y +  Math.Round( groupRect2.Height / 2.0), 208, 228, 228, 180);
        }
        else if (numArray2[index4] == 11)
        {
          DrawMod.drawLine( g, groupRect1.X +  Math.Round( groupRect1.Width / 2.0), groupRect1.Y + groupRect1.Height, groupRect2.X +  Math.Round( groupRect2.Width / 2.0), groupRect2.Y, 108, 108, 108, 180, 3);
          DrawMod.drawLine( g, groupRect1.X +  Math.Round( groupRect1.Width / 2.0), groupRect1.Y + groupRect1.Height, groupRect2.X +  Math.Round( groupRect2.Width / 2.0), groupRect2.Y, 208, 228, 228, 180);
        }
        else if (numArray1[index4] == numArray1[index3])
        {
          DrawMod.drawLine( g, groupRect1.X +  Math.Round( groupRect1.Width / 2.0), groupRect1.Y + groupRect1.Height, groupRect2.X +  Math.Round( groupRect2.Width / 2.0), groupRect2.Y, 108, 108, 108, 180, 3);
          DrawMod.drawLine( g, groupRect1.X +  Math.Round( groupRect1.Width / 2.0), groupRect1.Y + groupRect1.Height, groupRect2.X +  Math.Round( groupRect2.Width / 2.0), groupRect2.Y, 208, 228, 228, 180);
        }
        else
        {
          DrawMod.drawLine( g, groupRect1.X + groupRect1.Width, groupRect1.Y +  Math.Round( groupRect1.Height / 2.0), groupRect2.X, groupRect2.Y +  Math.Round( groupRect2.Height / 2.0), 108, 108, 108, 180, 3);
          DrawMod.drawLine( g, groupRect1.X + groupRect1.Width, groupRect1.Y +  Math.Round( groupRect1.Height / 2.0), groupRect2.X, groupRect2.Y +  Math.Round( groupRect2.Height / 2.0), 208, 228, 228, 180);
        }
      }
      let mut index6: i32 = 1;
      do
      {
        let mut idValue2: i32 = numArray3[index6];
        let mut tgroupX: i32 = numArray1[index6];
        let mut tgroupY: i32 = numArray2[index6];
        if (idValue2 > 0)
        {
          Rectangle groupRect = self.GetGroupRect(tgroupX, tgroupY);
          if (idValue2 == 206)
            groupRect.Width =  Math.Round( groupRect.Width * 1.5);
          data1: String = self.game.Data.StringListObj[stringListById1].GetData(0, idValue2, 1);
          data2: String = self.game.Data.StringListObj[stringListById1].GetData(0, idValue2, 6);
          bool flag1 = false;
          bool flag2 = false;
          let mut num6: i32 = 0;
          let mut num7: i32 = 0;
          SimpleList simpleList2 = SimpleList::new();
          str1: String = "";
          str2: String = "";
          let mut length: i32 = self.game.Data.StringListObj[stringListById2].Length;
          for (let mut index7: i32 = 0; index7 <= length; index7 += 1)
          {
            if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index7, 1])) == idValue2 &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index7, 2])) == id1)
            {
              let mut num8: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index7, 4]));
              let mut num9: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index7, 5]));
              if (num9 < 1 & num8 < 1)
                flag1 = true;
              if (num9 < 1 & num8 > 0)
              {
                flag1 = true;
                num6 += 1;
              }
              if (num9 > 0 & num8 > 0)
              {
                flag1 = true;
                num7 += 1;
                simpleList2.Add(num9, num9);
                str3: String = self.game.Data.StringListObj[stringListById2].Data[index7, 3];
                str1 = str1 + "• " + str3 + "\r\n";
              }
            }
          }
          if (!flag1)
          {
            eventRelatedObj: EventRelatedClass = self.game.EventRelatedObj;
            let mut id2: i32 = self.game.Data.StringListObj[stringListById4].ID;
            let mut id3: i32 = self.game.Data.StringListObj[stringListById3].ID;
            logicString: String = data2;
            Random random = (Random) null;
             Random local =  random;
            let mut num10: i32 = eventRelatedObj.CheckLogicStringStart(id2, id3, logicString, 0,  local);
            flag2 = true;
            if (num10 < 1)
              flag2 = false;
          }
          ttext: String;
          c: Color;
          num11: i32;
          if (flag2)
          {
            ttext = "This Model Type could be discovered";
            c = Color.LightGray;
            DrawMod.DrawBlockGradient2( g, groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height, Color.FromArgb(100, 50, 60, 50), Color.FromArgb(50, 20, 30, 20));
          }
          else if (flag1 & simpleList2.Counter > -1)
          {
            ttext = "This Model Type is already discovered and you have designed Models.";
            c = Color.White;
            DrawMod.DrawBlockGradient2( g, groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height, Color.FromArgb(100, 0, 150, 0), Color.FromArgb(50, 0, 75, 0));
          }
          else if (flag1 & simpleList2.Counter == -1)
          {
            ttext = "This Model Type is already discovered, but you did not complete design of any Model yet.";
            c = Color.LightGray;
            DrawMod.DrawBlockGradient2( g, groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height, Color.FromArgb(100, 50, 135, 50), Color.FromArgb(50, 25, 67, 25));
            if (num6 > 0)
              data1 += "*";
          }
          else
          {
            ttext = "This Model Type cannot be discovered yet.";
            c = Color.Gray;
            DrawMod.DrawBlockGradient2( g, groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height, self.game.MarcCol1, self.game.MarcCol2);
            str4: String = self.game.Data.StringListObj[stringListById1].GetData(0, idValue2, 6).ToLower();
            let mut num12: i32 = 1;
            do
            {
              if (Strings.InStr(str4, "tech.") > 0)
              {
                let mut Start: i32 = Strings.InStr(str4, "tech.");
                let mut num13: i32 = Strings.InStr(Start, str4, ">");
                num11 =  Math.Round(Conversion.Val(Strings.Mid(str4, Start + 5, num13 - (Start + 5))));
                if (num11 > 0)
                {
                  data3: String = self.game.Data.StringListObj[stringListById7].GetData(0, num11, 1);
                  str2 = str2 + "• " + data3 + "\r\n";
                }
                str4 = str4.Replace("tech." + num11.ToString(), "used." + num11.ToString());
              }
              num12 += 1;
            }
            while (num12 <= 4);
          }
          if (str2.Length > 0)
            ttext = ttext + "\r\n\r\nYou need to have mastered the following Techs:\r\n" + str2;
          if (str1.Length > 0)
            ttext = ttext + "\r\n\r\nYou have the following Models:\r\n" + str1;
          let mut num14: i32 =  Math.Round( (groupRect.Height - 19) / 3.0);
          let mut num15: i32 = 28;
          Rectangle trect1;
          Rectangle trect2;
          if (simpleList2.Counter == -1)
          {
            if (groupRect.Width < 180)
            {
              DrawMod.DrawTextColouredMarcCenter( g, data1, self.game.MarcFont5, groupRect.X +  Math.Round( groupRect.Width / 2.0), groupRect.Y - 6 +  Math.Round( groupRect.Height / 2.0), c);
              trect1 = Rectangle::new(groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height);
              trect2 = trect1;
              self.AddMouse( trect2, data1, ttext);
            }
            else
            {
              DrawMod.DrawTextColouredMarcCenter( g, data1, self.game.MarcFont4, groupRect.X +  Math.Round( groupRect.Width / 2.0), groupRect.Y - 9 +  Math.Round( groupRect.Height / 2.0), c);
              trect2 = Rectangle::new(groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height);
              trect1 = trect2;
              self.AddMouse( trect1, data1, ttext);
            }
          }
          else if (groupRect.Width < 200)
          {
            DrawMod.DrawTextColouredMarcCenter( g, data1, self.game.MarcFont5, groupRect.X +  Math.Round( groupRect.Width / 2.0), groupRect.Y + 3, c);
            num15 = 18;
            trect2 = Rectangle::new(groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height);
            trect1 = trect2;
            self.AddMouse( trect1, data1, ttext);
          }
          else
          {
            DrawMod.DrawTextColouredMarcCenter( g, data1, self.game.MarcFont4, groupRect.X +  Math.Round( groupRect.Width / 2.0), groupRect.Y + 5, c);
            trect2 = Rectangle::new(groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height);
            trect1 = trect2;
            self.AddMouse( trect1, data1, ttext);
          }
          if (simpleList2.Counter > -1)
          {
            SizeF sizeF2 = groupRect.Width >= 180 ? g.MeasureString(num7.ToString() + "x", self.game.MarcFont3) : g.MeasureString(num7.ToString() + "x", self.game.MarcFont4);
            simpleList2.ReverseSort();
            let mut num16: i32 = 1 + simpleList2.Counter;
            let mut num17: i32 = groupRect.Height - num15;
            let mut index8: i32 = 0;
            num11 = self.game.HandyFunctionsObj.GetSFTypeByID(simpleList2.Id[index8]);
            objBitmap: Bitmap = self.game.CustomBitmapObj.DrawSFTypeGraphic(num11, false, cultureGroupId, self.game.Data.Turn, -1);
            float num18 =  objBitmap.Width /  objBitmap.Height;
            let mut h: i32 = num17;
            let mut w: i32 =  Math.Round( (num18 *  num17));
            let mut x: i32 = groupRect.X +  Math.Round( groupRect.Width / 2.0 -  w / 2.0) +  Math.Round( (sizeF2.Width / 2f));
            let mut y: i32 = groupRect.Y - 5 + num15;
            DrawMod.DrawScaled( g,  objBitmap, x, y, w, h, true);
            objBitmap.Dispose();
            if (groupRect.Width < 180)
              DrawMod.DrawTextColouredMarcCenter( g, num7.ToString() + "x", self.game.MarcFont4, x -  Math.Round( (sizeF2.Width / 2f)), y + 1 +  Math.Round( h / 2.0) -  Math.Round( (sizeF2.Height / 2f)), c);
            else
              DrawMod.DrawTextColouredMarcCenter( g, num7.ToString() + "x", self.game.MarcFont3, x -  Math.Round( (sizeF2.Width / 2f)), y + 1 +  Math.Round( h / 2.0) -  Math.Round( (sizeF2.Height / 2f)), c);
          }
          DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, groupRect.X, groupRect.Y, groupRect.Width, groupRect.Height, -1, -1);
        }
        index6 += 1;
      }
      while (index6 <= 47);
      g.Dispose();
    }

    pub Rectangle GetGroupRect(tgroupX: i32, tgroupY: i32)
    {
      let mut num1: i32 = 50;
      let mut num2: i32 = 104;
      let mut num3: i32 = self.useHeight - 768;
      let mut num4: i32 =  Math.Round(170.0 + Math.Floor( (self.useWidth - 1280) / 7.0));
      let mut num5: i32 =  Math.Round(62.0 + Math.Floor( num3 / 12.0));
      let mut x: i32 = (tgroupX - 1) * num4 + num1;
      let mut y: i32 = (tgroupY - 1) * num5 + num2;
      let mut num6: i32 = num4 - 8;
      let mut height: i32 = num5 - 18;
      let mut width: i32 =  Math.Round( num6 - Math.Floor( num3 / 24.0));
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
