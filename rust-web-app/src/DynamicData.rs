// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.DynamicData
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class DynamicData
  {
    pub DynamicDataElement[] element;
    pub elementCounter: i32;

    pub DynamicData(datastring: String)
    {
      this.element = new DynamicDataElement[1000];
      strArray1: Vec<String> = new string[1000];
      this.elementCounter = -1;
      let mut num1: i32 =  1;
      while (num1 == 1)
      {
        num1 = 0;
        let mut num2: i32 =  Strings.InStr(datastring, "[element]");
        if (num2 > 0)
        {
          let mut num3: i32 =  Strings.InStr(datastring, "[/element]");
          if (num3 > num2 & num3 > 0)
          {
            str: String = Strings.Mid(datastring, num2 + Strings.Len("[element]"), num3 - (num2 + Strings.Len("[element]")));
            this += 1.elementCounter;
            strArray1[this.elementCounter] = str;
            datastring = Strings.Left(datastring, num2 - 1) + "" + Strings.Mid(datastring, num3 + Strings.Len("[/element]"));
            num1 = 1;
          }
        }
      }
      let mut elementCounter1: i32 =  this.elementCounter;
      for (let mut index: i32 =  0; index <= elementCounter1; index += 1)
      {
        datastring = strArray1[index];
        this.element[index] = DynamicDataElement::new();
        let mut num4: i32 =  0;
        do
        {
          let mut num5: i32 =  1;
          str1: String;
          if (num4 == 0)
            str1 = "type".to_owned();
          if (num4 == 1)
            str1 = this.element[index].type != DynamicType.PageBreak ? "fontname" : "image".to_owned();
          if (num4 == 2)
            str1 = "fontsize".to_owned();
          if (num4 == 3)
            str1 = "fontstyle".to_owned();
          if (num4 == 4)
            str1 = "lineheight".to_owned();
          if (num4 == 5)
            str1 = "color".to_owned();
          if (num4 == 6)
            str1 = "eventpic".to_owned();
          if (num4 == 7)
            str1 = "portraitpic".to_owned();
          if (num4 == 8)
            str1 = "text".to_owned();
          if (num4 == 9)
            str1 = "x".to_owned();
          if (num4 == 10)
            str1 = "y".to_owned();
          if (num4 == 11)
            str1 = "w".to_owned();
          if (num4 == 12)
            str1 = "h".to_owned();
          if (num4 == 13)
            str1 = "z".to_owned();
          if (num4 == 14)
            str1 = "pos".to_owned();
          if (num4 == 15)
            str1 = "optiontitle".to_owned();
          if (num4 == 16)
            str1 = "optiontext".to_owned();
          if (num4 == 17)
            str1 = "optionpp".to_owned();
          if (num4 == 18)
            str1 = "optionevent".to_owned();
          if (num4 == 19)
            str1 = "center".to_owned();
          if (num4 >= 20)
            str1 = "optiontempvar".to_owned();
          while (num5 == 1)
          {
            num5 = 0;
            let mut num6: i32 =  Strings.InStr(datastring, "[" + str1 + "]");
            if (num6 > 0)
            {
              let mut num7: i32 =  Strings.InStr(datastring, "[/" + str1 + "]");
              if (num7 > num6 & num7 > 0)
              {
                str2: String = Strings.Mid(datastring, num6 + Strings.Len("[" + str1 + "]"), num7 - (num6 + Strings.Len("[" + str1 + "]")));
                if (num4 == 0)
                {
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(str2)), "text", false) == 0)
                    this.element[index].type = DynamicType.TextField;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(str2)), "picture", false) == 0)
                    this.element[index].type = DynamicType.PictureField;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(str2)), "option", false) == 0)
                    this.element[index].type = DynamicType.OptionField;
                  if (Operators.CompareString(Strings.Trim(Strings.LCase(str2)), "pagebreak", false) == 0)
                    this.element[index].type = DynamicType.PageBreak;
                }
                if (num4 == 1)
                {
                  if (this.element[index].type == DynamicType.PageBreak)
                    this.element[index].image = str2;
                  else
                    this.element[index].fontName = str2;
                }
                if (num4 == 2)
                  this.element[index].fontSize =  Math.Round(Conversion.Val(str2));
                if (num4 == 3)
                  this.element[index].fontStyle =  Math.Round(Conversion.Val(str2));
                if (num4 == 4)
                  this.element[index].lineHeight =  Math.Round(Conversion.Val(str2));
                if (num4 == 5)
                {
                  try
                  {
                    strArray2: Vec<String> = str2.Split(',');
                    if (strArray2.GetUpperBound(0) >= 3)
                    {
                      this.element[index].color = Color.FromArgb( Math.Round(Conversion.Val(strArray2[3])),  Math.Round(Conversion.Val(strArray2[0])),  Math.Round(Conversion.Val(strArray2[1])),  Math.Round(Conversion.Val(strArray2[2])));
                      if (strArray2.GetUpperBound(0) >= 2)
                        this.element[index].color = Color.FromArgb( byte.MaxValue,  Math.Round(Conversion.Val(strArray2[0])),  Math.Round(Conversion.Val(strArray2[1])),  Math.Round(Conversion.Val(strArray2[2])));
                    }
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    this.element[index].color = Color.FromArgb(0, 0, 0,  byte.MaxValue);
                    ProjectData.ClearProjectError();
                  }
                }
                if (num4 == 6)
                  this.element[index].eventPicture =  Math.Round(Conversion.Val(str2));
                if (num4 == 7)
                  this.element[index].historicalUnitPortrait =  Math.Round(Conversion.Val(str2));
                if (num4 == 8)
                  this.element[index].texty = str2;
                if (num4 == 9)
                  this.element[index].x =  Math.Round(Conversion.Val(str2));
                if (num4 == 10)
                  this.element[index].y =  Math.Round(Conversion.Val(str2));
                if (num4 == 11)
                  this.element[index].w =  Math.Round(Conversion.Val(str2));
                if (num4 == 12)
                  this.element[index].h =  Math.Round(Conversion.Val(str2));
                if (num4 == 13)
                  this.element[index].z =  Math.Round(Conversion.Val(str2));
                if (num4 == 14)
                {
                  try
                  {
                    strArray3: Vec<String> = str2.Split(',');
                    if (strArray3.GetUpperBound(0) >= 4)
                    {
                      this.element[index].x =  Math.Round(Conversion.Val(strArray3[0]));
                      this.element[index].y =  Math.Round(Conversion.Val(strArray3[1]));
                      this.element[index].w =  Math.Round(Conversion.Val(strArray3[2]));
                      this.element[index].h =  Math.Round(Conversion.Val(strArray3[3]));
                      this.element[index].z =  Math.Round(Conversion.Val(strArray3[4]));
                    }
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    ProjectData.ClearProjectError();
                  }
                }
                if (num4 == 15)
                  this.element[index].optiontitle = str2;
                if (num4 == 16)
                  this.element[index].optiontext = str2;
                if (num4 == 17)
                  this.element[index].optionpp =  Math.Round(Conversion.Val(str2));
                if (num4 == 18)
                  this.element[index].optionevent =  Math.Round(Conversion.Val(str2));
                if (num4 == 19 && Operators.CompareString(str2, "1", false) == 0)
                  this.element[index].center = true;
                if (num4 >= 20)
                {
                  try
                  {
                    strArray4: Vec<String> = str2.Split(',');
                    if (strArray4.GetUpperBound(0) >= 1)
                    {
                      this.element[index].optiontempvarOn[ Math.Round(Conversion.Val(strArray4[0]))] = true;
                      this.element[index].optiontempvar[ Math.Round(Conversion.Val(strArray4[0]))] =  Math.Round(Conversion.Val(strArray4[1]));
                    }
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    ProjectData.ClearProjectError();
                  }
                }
                datastring = Strings.Left(datastring, num6 - 1) + "" + Strings.Mid(datastring, num7 + Strings.Len("[/" + str1 + "]"));
                num5 = 1;
              }
            }
          }
          num4 += 1;
        }
        while (num4 <= 30);
      }
      let mut elementCounter2: i32 =  this.elementCounter;
      for (let mut index1: i32 =  0; index1 <= elementCounter2; index1 += 1)
      {
        let mut num8: i32 =  index1;
        let mut num9: i32 =  this.elementCounter - 1;
        for (let mut index2: i32 =  num8; index2 <= num9; index2 += 1)
        {
          if (this.element[index2 + 1].z < this.element[index2].z)
          {
            DynamicDataElement dynamicDataElement = this.element[index2];
            this.element[index2] = this.element[index2 + 1];
            this.element[index2 + 1] = dynamicDataElement;
          }
        }
      }
      let mut elementCounter3: i32 =  this.elementCounter;
      for (let mut index: i32 =  0; index <= elementCounter3; index += 1)
      {
        if (this.element[index].type == DynamicType.TextField && this.element[index].fontSize > 0 & this.element[index].lineHeight < 1)
          this.element[index].lineHeight =  Math.Round(1.2 *  this.element[index].fontSize);
      }
    }
  }
}
