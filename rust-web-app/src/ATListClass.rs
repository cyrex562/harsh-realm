// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ATListClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class ATListClass
  {
    pub ListCount: i32;
    pub ListName: Vec<String>;
    pub ListData: Vec<i32>;
    pub ListValue: Vec<String>;
    pub ListValue2: Vec<String>;
    pub ListValue3: Vec<String>;
    pub ListValue4: Vec<String>;
    pub ListColor: Vec<i32>;
    pub ListBmp: Vec<Bitmap>;
    pub ListR: Vec<i32>;
    pub ListG: Vec<i32>;
    pub ListB: Vec<i32>;

    pub ATListClass()
    {
      this.ListName = new string[1];
      this.ListData = new int[1];
      this.ListValue = new string[1];
      this.ListValue2 = new string[1];
      this.ListValue3 = new string[1];
      this.ListValue4 = new string[1];
      this.ListColor = new int[1];
      this.ListBmp = new Bitmap[1];
      this.ListR = new int[1];
      this.ListG = new int[1];
      this.ListB = new int[1];
      this.ListCount = -1;
    }

    pub void add(
      tname: String,
      int tdata,
      tvalue: String = "",
      tvalue2: String = "",
      tvalue3: String = "",
      tvalue4: String = "",
      let mut tcol: i32 =  -1,
      tbmp: Bitmap = null,
      let mut tr: i32 =  -1,
      let mut tg: i32 =  -1,
      let mut tb: i32 =  -1)
    {
      this += 1.ListCount;
      this.ListName = (string[]) Utils.CopyArray((Array) this.ListName, (Array) new string[this.ListCount + 1]);
      this.ListData = (int[]) Utils.CopyArray((Array) this.ListData, (Array) new int[this.ListCount + 1]);
      this.ListValue = (string[]) Utils.CopyArray((Array) this.ListValue, (Array) new string[this.ListCount + 1]);
      this.ListValue2 = (string[]) Utils.CopyArray((Array) this.ListValue2, (Array) new string[this.ListCount + 1]);
      this.ListValue3 = (string[]) Utils.CopyArray((Array) this.ListValue3, (Array) new string[this.ListCount + 1]);
      this.ListValue4 = (string[]) Utils.CopyArray((Array) this.ListValue4, (Array) new string[this.ListCount + 1]);
      this.ListColor = (int[]) Utils.CopyArray((Array) this.ListColor, (Array) new int[this.ListCount + 1]);
      this.ListBmp = (Bitmap[]) Utils.CopyArray((Array) this.ListBmp, (Array) new Bitmap[this.ListCount + 1]);
      this.ListR = (int[]) Utils.CopyArray((Array) this.ListR, (Array) new int[this.ListCount + 1]);
      this.ListG = (int[]) Utils.CopyArray((Array) this.ListG, (Array) new int[this.ListCount + 1]);
      this.ListB = (int[]) Utils.CopyArray((Array) this.ListB, (Array) new int[this.ListCount + 1]);
      this.ListName[this.ListCount] = tname;
      this.ListData[this.ListCount] = tdata;
      this.ListValue[this.ListCount] = tvalue;
      this.ListValue2[this.ListCount] = tvalue2;
      this.ListValue3[this.ListCount] = tvalue3;
      this.ListValue4[this.ListCount] = tvalue4;
      this.ListColor[this.ListCount] = tcol;
      this.ListBmp[this.ListCount] = tbmp;
      this.ListR[this.ListCount] = tr;
      this.ListG[this.ListCount] = tg;
      this.ListB[this.ListCount] = tb;
    }

    pub fn Sort()
    {
      if (this.ListCount < 1)
        return;
      let mut num1: i32 =  this.ListCount - 1;
      for (let mut index1: i32 =  0; index1 <= num1; index1 += 1)
      {
        let mut num2: i32 =  this.ListCount - 1;
        for (let mut index2: i32 =  0; index2 <= num2; index2 += 1)
        {
          if (Operators.CompareString(this.ListName[index2], this.ListName[index2 + 1], false) > 0)
          {
            str1: String = this.ListName[index2 + 1];
            let mut num3: i32 =  this.ListData[index2 + 1];
            str2: String = this.ListValue[index2 + 1];
            str3: String = this.ListValue2[index2 + 1];
            str4: String = this.ListValue3[index2 + 1];
            str5: String = this.ListValue4[index2 + 1];
            this.ListName[index2 + 1] = this.ListName[index2];
            this.ListData[index2 + 1] = this.ListData[index2];
            this.ListValue[index2 + 1] = this.ListValue[index2];
            this.ListValue2[index2 + 1] = this.ListValue2[index2];
            this.ListValue3[index2 + 1] = this.ListValue3[index2];
            this.ListValue4[index2 + 1] = this.ListValue4[index2];
            this.ListName[index2] = str1;
            this.ListData[index2] = num3;
            this.ListValue[index2] = str2;
            this.ListValue2[index2] = str3;
            this.ListValue3[index2] = str4;
            this.ListValue4[index2] = str5;
          }
        }
      }
    }

    pub int SortWithRef(int TempInt)
    {
      if (this.ListCount < 1)
      {
        int num;
        return num;
      }
      let mut num1: i32 =  this.ListCount - 1;
      for (let mut index1: i32 =  0; index1 <= num1; index1 += 1)
      {
        let mut num2: i32 =  this.ListCount - 1;
        for (let mut index2: i32 =  0; index2 <= num2; index2 += 1)
        {
          if (Operators.CompareString(this.ListName[index2], this.ListName[index2 + 1], false) > 0)
          {
            str: String = this.ListName[index2 + 1];
            let mut num3: i32 =  this.ListData[index2 + 1];
            if (TempInt == index2 + 1)
              TempInt = index2;
            else if (TempInt == index2)
              TempInt = index2 + 1;
            this.ListName[index2 + 1] = this.ListName[index2];
            this.ListData[index2 + 1] = this.ListData[index2];
            this.ListName[index2] = str;
            this.ListData[index2] = num3;
          }
        }
      }
      return TempInt;
    }
  }
}
