// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UDSData
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class UDSData
  {
    pub UDSElement[] element;
    pub elementCounter: i32;

    pub UDSData(datastring: String, bool allGray)
    {
      self.element = new UDSElement[1000];
      strArray1: Vec<String> = new string[10000];
      self.elementCounter = -1;
      let mut num1: i32 = 1;
      while (num1 == 1)
      {
        num1 = 0;
        let mut num2: i32 = Strings.InStr(datastring, "[element]");
        if (num2 > 0)
        {
          let mut num3: i32 = Strings.InStr(datastring, "[/element]");
          if (num3 > num2 & num3 > 0)
          {
            str: String = Strings.Mid(datastring, num2 + Strings.Len("[element]"), num3 - (num2 + Strings.Len("[element]")));
            if (self.elementCounter < 999)
            {
              self += 1.elementCounter;
              strArray1[self.elementCounter] = str;
            }
            datastring = Strings.Left(datastring, num2 - 1) + "" + Strings.Mid(datastring, num3 + Strings.Len("[/element]"));
            num1 = 1;
          }
        }
      }
      let mut elementCounter1: i32 = self.elementCounter;
      for (let mut index: i32 = 0; index <= elementCounter1; index += 1)
      {
        datastring = strArray1[index];
        self.element[index] = UDSElement::new();
        let mut num4: i32 = 0;
        do
        {
          let mut num5: i32 = 1;
          str: String;
          if (num4 == 0)
            str = "type";
          if (num4 == 1)
            str = self.element[index].type != UDSType.PageBreak ? "fontname" : "image";
          if (num4 == 2)
            str = "fontsize";
          if (num4 == 3)
            str = "fontstyle";
          if (num4 == 4)
            str = "lineheight";
          if (num4 == 5)
            str = "color";
          if (num4 == 6)
            str = "eventpic";
          if (num4 == 7)
            str = "portraitpic";
          if (num4 == 8)
            str = "text";
          if (num4 == 9)
            str = "x";
          if (num4 == 10)
            str = "y";
          if (num4 == 11)
            str = "w";
          if (num4 == 12)
            str = "h";
          if (num4 == 13)
            str = "z";
          if (num4 == 14)
            str = "pos";
          if (num4 == 15)
            str = "optiontitle";
          if (num4 == 16)
            str = "optiontext";
          if (num4 == 17)
            str = "optionpp";
          if (num4 == 18)
            str = "optionevent";
          if (num4 >= 19 & num4 <= 29)
            str = "optiontempvar";
          if (num4 == 30)
            str = "event";
          if (num4 == 31)
            str = "mouseover";
          if (num4 == 32)
            str = "key";
          if (num4 == 33)
            str = "value";
          if (num4 == 34)
            str = "grayed";
          if (num4 == 35)
            str = "group";
          if (num4 == 36)
            str = "flagged";
          if (num4 == 37)
            str = "smallgfx";
          if (num4 == 38)
            str = "minvalue";
          if (num4 == 39)
            str = "maxvalue";
          if (num4 == 40)
            str = "temppic";
          if (num4 == 41)
            str = "rotation";
          if (num4 == 42)
            str = "center";
          if (num4 == 43)
            str = "subtype";
          if (num4 == 44)
            str = "ox";
          if (num4 == 45)
            str = "oy";
          if (num4 == 46)
            str = "ow";
          if (num4 == 47)
            str = "oh";
          while (num5 == 1)
          {
            num5 = 0;
            let mut num6: i32 = Strings.InStr(datastring, "[" + str + "]");
            if (num6 > 0)
            {
              let mut num7: i32 = Strings.InStr(datastring, "[/" + str + "]");
              if (num7 > num6 & num7 > 0)
              {
                InputStr: String = Strings.Mid(datastring, num6 + Strings.Len("[" + str + "]"), num7 - (num6 + Strings.Len("[" + str + "]")));
                if (num4 == 0)
                {
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "text", false) == 0)
                    self.element[index].type = UDSType.TextField;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "picture", false) == 0)
                    self.element[index].type = UDSType.PictureField;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "option", false) == 0)
                    self.element[index].type = UDSType.OptionField;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "pagebreak", false) == 0)
                    self.element[index].type = UDSType.PageBreak;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "button", false) == 0)
                    self.element[index].type = UDSType.Button;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "flag", false) == 0)
                    self.element[index].type = UDSType.Flag;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "hidden", false) == 0)
                    self.element[index].type = UDSType.Hidden;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "layout", false) == 0)
                    self.element[index].type = UDSType.Layout;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "slider", false) == 0)
                    self.element[index].type = UDSType.Slider;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "wav", false) == 0)
                    self.element[index].type = UDSType.Wav;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "table", false) == 0)
                    self.element[index].type = UDSType.Table;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "line", false) == 0)
                    self.element[index].type = UDSType.Line;
                }
                if (self.element[index].type == UDSType.Table & num4 == 12)
                  InputStr = ( Math.Round(Conversion.Val(InputStr)) - 40).ToString();
                if (num4 == 1)
                {
                  if (self.element[index].type == UDSType.PageBreak)
                    self.element[index].image = InputStr;
                  else
                    self.element[index].fontName = InputStr;
                }
                if (num4 == 2)
                  self.element[index].fontSize =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 3)
                  self.element[index].fontStyle =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 4)
                  self.element[index].lineHeight =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 5)
                {
                  try
                  {
                    strArray2: Vec<String> = InputStr.Split(',');
                    if (strArray2.GetUpperBound(0) >= 3)
                      self.element[index].color = Color.FromArgb( Math.Round(Conversion.Val(strArray2[3])),  Math.Round(Conversion.Val(strArray2[0])),  Math.Round(Conversion.Val(strArray2[1])),  Math.Round(Conversion.Val(strArray2[2])));
                    else if (strArray2.GetUpperBound(0) >= 2)
                      self.element[index].color = Color.FromArgb( byte.MaxValue,  Math.Round(Conversion.Val(strArray2[0])),  Math.Round(Conversion.Val(strArray2[1])),  Math.Round(Conversion.Val(strArray2[2])));
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    self.element[index].color = Color.FromArgb(0, 0, 0,  byte.MaxValue);
                    ProjectData.ClearProjectError();
                  }
                }
                if (num4 == 6)
                  self.element[index].eventPicture =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 7)
                  self.element[index].historicalUnitPortrait =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 8)
                {
                  self.element[index].texty = InputStr;
                  if (self.element[index].type == UDSType.Button && Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "exit", false) == 0 | Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "cancel", false) == 0 | Operators.CompareString(Strings.Trim(Strings.LCase(InputStr)), "back", false) == 0)
                    self.element[index].hidden = true;
                }
                if (num4 == 9)
                  self.element[index].x =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 10)
                  self.element[index].y =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 11)
                  self.element[index].w =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 12)
                  self.element[index].h =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 13)
                  self.element[index].z =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 14)
                {
                  try
                  {
                    strArray3: Vec<String> = InputStr.Split(',');
                    if (strArray3.GetUpperBound(0) >= 4)
                    {
                      self.element[index].x =  Math.Round(Conversion.Val(strArray3[0]));
                      self.element[index].y =  Math.Round(Conversion.Val(strArray3[1]));
                      self.element[index].w =  Math.Round(Conversion.Val(strArray3[2]));
                      self.element[index].h =  Math.Round(Conversion.Val(strArray3[3]));
                      self.element[index].z =  Math.Round(Conversion.Val(strArray3[4]));
                    }
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    ProjectData.ClearProjectError();
                  }
                }
                if (num4 == 15)
                  self.element[index].optiontitle = InputStr;
                if (num4 == 16)
                  self.element[index].optiontext = InputStr;
                if (num4 == 17)
                  self.element[index].optionpp =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 18)
                  self.element[index].optionevent =  Math.Round(Conversion.Val(InputStr));
                if (num4 >= 19 & num4 <= 29)
                {
                  try
                  {
                    strArray4: Vec<String> = InputStr.Split(',');
                    if (strArray4.GetUpperBound(0) >= 1)
                    {
                      self.element[index].optiontempvarOn[ Math.Round(Conversion.Val(strArray4[0]))] = true;
                      self.element[index].optiontempvar[ Math.Round(Conversion.Val(strArray4[0]))] =  Math.Round(Conversion.Val(strArray4[1]));
                    }
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    ProjectData.ClearProjectError();
                  }
                }
                if (num4 == 30)
                  self.element[index].eventNr =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 31)
                  self.element[index].mouseOver = InputStr;
                if (num4 == 32)
                  self.element[index].key = InputStr;
                if (num4 == 33)
                  self.element[index].value = InputStr;
                if (num4 == 34)
                  self.element[index].grayed =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 35)
                  self.element[index].group =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 36)
                  self.element[index].flagged = Conversion.Val(InputStr) == 1.0;
                if (num4 == 37)
                  self.element[index].smallgfx =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 38)
                  self.element[index].minvalue =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 39)
                  self.element[index].maxvalue =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 40)
                  self.element[index].tempPicture = InputStr;
                if (num4 == 41)
                  self.element[index].rotation =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 42)
                  self.element[index].center =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 43)
                  self.element[index].subtype =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 44)
                  self.element[index].ox =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 45)
                  self.element[index].oy =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 46)
                  self.element[index].ow =  Math.Round(Conversion.Val(InputStr));
                if (num4 == 47)
                  self.element[index].oh =  Math.Round(Conversion.Val(InputStr));
                datastring = Strings.Left(datastring, num6 - 1) + "" + Strings.Mid(datastring, num7 + Strings.Len("[/" + str + "]"));
                num5 = 1;
              }
            }
          }
          num4 += 1;
        }
        while (num4 <= 47);
      }
      let mut num8: i32 = self.elementCounter - 1;
      for (let mut index1: i32 = 0; index1 <= num8; index1 += 1)
      {
        if (self.element[index1].type == UDSType.Layout)
        {
          let mut num9: i32 = index1 + 1;
          let mut elementCounter2: i32 = self.elementCounter;
          for (let mut index2: i32 = num9; index2 <= elementCounter2; index2 += 1)
          {
            if (self.element[index2].type != UDSType.Hidden & self.element[index2].type != UDSType.Layout)
            {
              UDSElement[] element1 = self.element;
              UDSElement[] udsElementArray1 = element1;
              let mut index3: i32 = index2;
              let mut index4: i32 = index3;
              udsElementArray1[index4].x = element1[index3].x + self.element[index1].w;
              UDSElement[] element2 = self.element;
              UDSElement[] udsElementArray2 = element2;
              let mut index5: i32 = index2;
              let mut index6: i32 = index5;
              udsElementArray2[index6].y = element2[index5].y + self.element[index1].h;
              if (self.element[index2].type == UDSType.Line)
              {
                UDSElement[] element3 = self.element;
                UDSElement[] udsElementArray3 = element3;
                let mut index7: i32 = index2;
                let mut index8: i32 = index7;
                udsElementArray3[index8].w = element3[index7].w + self.element[index1].w;
                UDSElement[] element4 = self.element;
                UDSElement[] udsElementArray4 = element4;
                let mut index9: i32 = index2;
                let mut index10: i32 = index9;
                udsElementArray4[index10].h = element4[index9].h + self.element[index1].h;
              }
            }
          }
        }
      }
      let mut elementCounter3: i32 = self.elementCounter;
      for (let mut index11: i32 = 0; index11 <= elementCounter3; index11 += 1)
      {
        let mut num10: i32 = index11;
        let mut num11: i32 = self.elementCounter - 1;
        for (let mut index12: i32 = num10; index12 <= num11; index12 += 1)
        {
          if (self.element[index12 + 1].z < self.element[index12].z)
          {
            UDSElement udsElement = self.element[index12];
            self.element[index12] = self.element[index12 + 1];
            self.element[index12 + 1] = udsElement;
          }
        }
      }
      let mut elementCounter4: i32 = self.elementCounter;
      for (let mut index: i32 = 0; index <= elementCounter4; index += 1)
      {
        if (allGray)
          self.element[index].grayed = 1;
        if (self.element[index].type == UDSType.Slider && self.element[index].h < 1)
          self.element[index].h = 40;
        if (self.element[index].type == UDSType.PictureField)
        {
          if (self.element[index].eventPicture >= 2000000)
          {
            self.element[index].customBitmapFunction3 = self.element[index].eventPicture - 2000000;
            self.element[index].eventPicture = -1;
          }
          else if (self.element[index].eventPicture >= 1000000)
          {
            self.element[index].customBitmapFunction2 = self.element[index].eventPicture - 1000000;
            self.element[index].eventPicture = -1;
          }
          else if (self.element[index].eventPicture >= 100000)
          {
            self.element[index].customBitmapFunction = self.element[index].eventPicture - 100000;
            self.element[index].eventPicture = -1;
          }
          else if (self.element[index].eventPicture > 10000)
          {
            self.element[index].bitmapSlot = self.element[index].eventPicture - 10000;
            self.element[index].eventPicture = -1;
            if (self.element[index].w == 0 & self.element[index].h == 0)
            {
              self.element[index].w = BitmapStore.GetWidth(self.element[index].bitmapSlot);
              self.element[index].h = BitmapStore.Getheight(self.element[index].bitmapSlot);
            }
            else if (self.element[index].w == 0)
              self.element[index].w =  Math.Round( BitmapStore.GetWidth(self.element[index].bitmapSlot) * ( self.element[index].h /  BitmapStore.Getheight(self.element[index].bitmapSlot)));
            else if (self.element[index].h == 0)
              self.element[index].h =  Math.Round( BitmapStore.Getheight(self.element[index].bitmapSlot) * ( self.element[index].w /  BitmapStore.GetWidth(self.element[index].bitmapSlot)));
          }
          else if (self.element[index].eventPicture > -1)
          {
            if (self.element[index].w == 0 & self.element[index].h == 0)
            {
              self.element[index].w = BitmapStore.GetWidth(DrawMod.TGame.Data.EventPicNr[self.element[index].eventPicture]);
              self.element[index].h = BitmapStore.Getheight(DrawMod.TGame.Data.EventPicNr[self.element[index].eventPicture]);
            }
            else if (self.element[index].w == 0)
              self.element[index].w =  Math.Round( (BitmapStore.GetWidth(DrawMod.TGame.Data.EventPicNr[self.element[index].eventPicture]) * self.element[index].h) /  BitmapStore.Getheight(DrawMod.TGame.Data.EventPicNr[self.element[index].eventPicture]));
            else if (self.element[index].h == 0)
              self.element[index].h =  Math.Round( (BitmapStore.Getheight(DrawMod.TGame.Data.EventPicNr[self.element[index].eventPicture]) * self.element[index].w) /  BitmapStore.GetWidth(DrawMod.TGame.Data.EventPicNr[self.element[index].eventPicture]));
          }
        }
        if (self.element[index].type == UDSType.Flag && self.element[index].w == 0 & self.element[index].h == 0)
        {
          self.element[index].w = 35;
          self.element[index].h = 35;
        }
        if (self.element[index].type == UDSType.Table)
        {
          self.AddChildButtonForTable("<<", index, 1, self.element[index].x, self.element[index].y + self.element[index].h + 5, 50, 30);
          self.AddChildButtonForTable("<", index, 2, self.element[index].x + 60, self.element[index].y + self.element[index].h + 5, 50, 30);
          self.AddChildTextForTable("Page 1/" + self.GetTablePages(index).ToString(), index, 5, self.element[index].x + 120, self.element[index].y + self.element[index].h + 5, self.element[index].w - 240, 30);
          self.AddChildButtonForTable(">", index, 3, self.element[index].x + self.element[index].w - 110, self.element[index].y + self.element[index].h + 5, 50, 30);
          self.AddChildButtonForTable(">>", index, 4, self.element[index].x + self.element[index].w - 50, self.element[index].y + self.element[index].h + 5, 50, 30);
          self.element[index].rowsPerPage = self.GetRowsPerPage(index);
          self.element[index].topRow = 0;
          self.element[index].totalRows = self.GetTotalRows(index);
          index = index;
        }
      }
      let mut elementCounter5: i32 = self.elementCounter;
      for (let mut index: i32 = 0; index <= elementCounter5; index += 1)
      {
        if (self.element[index].type == UDSType.Slider & self.element[index].group > 0)
        {
          self += 1.elementCounter;
          let mut elementCounter6: i32 = self.elementCounter;
          self.element[elementCounter6] = UDSElement::new();
          self.element[elementCounter6].type = UDSType.Flag;
          self.element[elementCounter6].key = "SysAutoLock_" + index.ToString();
          self.element[elementCounter6].value = Conversions.ToString(0);
          self.element[elementCounter6].group = 0;
          self.element[elementCounter6].x = self.element[index].x + self.element[index].w + 10;
          self.element[elementCounter6].y = self.element[index].y + 5;
          self.element[elementCounter6].w = 35;
          self.element[elementCounter6].h = 35;
          self.element[index].rotation = elementCounter6;
          self.element[elementCounter6].mouseOver = "Flag to lock the Slider";
        }
      }
    }

    pub void AddChildButtonForTable(
      texty: String,
      tparentSlot: i32,
      tChildType: i32,
      tx: i32,
      ty: i32,
      tw: i32,
      th: i32)
    {
      self += 1.elementCounter;
      let mut elementCounter: i32 = self.elementCounter;
      self.element[elementCounter] = UDSElement::new();
      self.element[elementCounter].type = UDSType.Button;
      self.element[elementCounter].x = tx;
      self.element[elementCounter].y = ty;
      self.element[elementCounter].w = tw;
      self.element[elementCounter].h = th;
      self.element[elementCounter].texty = texty;
      self.element[elementCounter].fontName = self.element[tparentSlot].fontName;
      self.element[elementCounter].fontStyle = self.element[tparentSlot].fontStyle;
      self.element[elementCounter].fontSize = self.element[tparentSlot].fontSize;
      self.element[elementCounter].parentElement = tparentSlot;
      self.element[elementCounter].childType = tChildType;
      self.element[elementCounter].childData = self.GetRowsPerPage(tparentSlot);
      if (tChildType == 1)
        self.element[elementCounter].grayed = 1;
      if (tChildType == 2)
        self.element[elementCounter].grayed = 1;
      if (tChildType == 3 && 0 > self.GetTotalRows(tparentSlot) - self.GetRowsPerPage(tparentSlot) - 1)
        self.element[elementCounter].grayed = 1;
      if (tChildType != 4 || 0 <= self.GetTotalRows(tparentSlot) - self.GetRowsPerPage(tparentSlot) - 1)
        return;
      self.element[elementCounter].grayed = 1;
    }

    pub void AddChildTextForTable(
      texty: String,
      tparentSlot: i32,
      tChildType: i32,
      tx: i32,
      ty: i32,
      tw: i32,
      th: i32)
    {
      self += 1.elementCounter;
      let mut elementCounter: i32 = self.elementCounter;
      self.element[elementCounter] = UDSElement::new();
      self.element[elementCounter].type = UDSType.TextField;
      self.element[elementCounter].x = tx;
      self.element[elementCounter].y = ty;
      self.element[elementCounter].w = tw;
      self.element[elementCounter].h = th;
      self.element[elementCounter].center = 1;
      self.element[elementCounter].texty = texty;
      self.element[elementCounter].fontName = self.element[tparentSlot].fontName;
      self.element[elementCounter].fontStyle = self.element[tparentSlot].fontStyle;
      self.element[elementCounter].fontSize = self.element[tparentSlot].fontSize;
      self.element[elementCounter].parentElement = tparentSlot;
      self.element[elementCounter].childType = tChildType;
      self.element[elementCounter].lineHeight = 24;
      self.element[elementCounter].color = Color.Black;
    }

    pub fn GetTablePages(slot: i32) -> i32
    {
      StringListClass stringListClass = DrawMod.TGame.Data.StringListObj[ Math.Round(Conversion.Val(self.element[slot].texty))];
      let mut num1: i32 = stringListClass.Length + 1;
      let mut num2: i32 =  Math.Round(Math.Floor( self.element[slot].h /  self.element[slot].lineHeight)) - 1;
      return  Math.Round(Math.Floor( Math.Max(0, stringListClass.Length - 1) /  num2)) + 1;
    }

    pub fn GetRowsPerPage(slot: i32) -> i32
    {
      let mut num: i32 = DrawMod.TGame.Data.StringListObj[ Math.Round(Conversion.Val(self.element[slot].texty))].Length + 1;
      return  Math.Round(Math.Floor( self.element[slot].h /  self.element[slot].lineHeight)) - 1;
    }

    pub fn GetTotalRows(slot: i32) => DrawMod.TGame.Data.StringListObj[ Math.Round(Conversion.Val(self.element[slot].texty)) -> i32].Length + 1;
  }
}
