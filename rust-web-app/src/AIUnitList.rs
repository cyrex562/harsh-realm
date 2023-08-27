// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.AIUnitList
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;

namespace WindowsApplication1
{
  pub class AIUnitList
  {
    pub counter: i32;
    pub unr: Vec<i32>;
    pub AIid: Vec<i32>;
    pub data: Vec<i32>;
    pub data2: Vec<i32>;
    pub maxcounter: i32;

    pub AIUnitList()
    {
      this.unr = new int[21];
      this.AIid = new int[21];
      this.data = new int[21];
      this.data2 = new int[21];
      this.counter = -1;
      this.maxcounter = 20;
    }

    pub fn add(int tunr, int tAIid, let mut tdata: i32 =  -1, let mut tdata2: i32 =  -1)
    {
      this += 1.counter;
      if (this.counter > this.maxcounter)
      {
        this.maxcounter += 40;
        this.unr = (int[]) Utils.CopyArray((Array) this.unr, (Array) new int[this.maxcounter + 1]);
        this.AIid = (int[]) Utils.CopyArray((Array) this.AIid, (Array) new int[this.maxcounter + 1]);
        this.data = (int[]) Utils.CopyArray((Array) this.data, (Array) new int[this.maxcounter + 1]);
        this.data2 = (int[]) Utils.CopyArray((Array) this.data2, (Array) new int[this.maxcounter + 1]);
      }
      this.unr[this.counter] = tunr;
      this.AIid[this.counter] = tAIid;
      this.data[this.counter] = tdata;
      this.data2[this.counter] = tdata2;
    }

    pub fn removeUnr(int tunr)
    {
      if (this.counter <= -1)
        return;
      let mut num1: i32 =  -1;
      let mut counter: i32 =  this.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.unr[index] == tunr)
          num1 = index;
      }
      if (num1 > -1 && num1 < this.counter)
      {
        let mut num2: i32 =  num1;
        let mut num3: i32 =  this.counter - 1;
        for (let mut index: i32 =  num2; index <= num3; index += 1)
        {
          this.unr[index] = this.unr[index + 1];
          this.AIid[index] = this.AIid[index + 1];
          this.data[index] = this.data[index + 1];
          this.data2[index] = this.data2[index + 1];
        }
      }
      --this.counter;
    }

    pub fn removeAiId(int tAIid)
    {
      if (this.counter <= -1)
        return;
      let mut num1: i32 =  -1;
      let mut counter: i32 =  this.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.AIid[index] == tAIid)
          num1 = index;
      }
      if (num1 > -1 && num1 < this.counter)
      {
        let mut num2: i32 =  num1;
        let mut num3: i32 =  this.counter - 1;
        for (let mut index: i32 =  num2; index <= num3; index += 1)
        {
          this.unr[index] = this.unr[index + 1];
          this.AIid[index] = this.AIid[index + 1];
          this.data[index] = this.data[index + 1];
          this.data2[index] = this.data2[index + 1];
        }
      }
      --this.counter;
    }

    pub CheckIfPresentUnr: bool(int tunr)
    {
      if (this.counter <= -1)
        return false;
      let mut num: i32 =  -1;
      let mut counter: i32 =  this.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.unr[index] == tunr)
          num = index;
      }
      return num > -1;
    }

    pub int FindUnrSlot(int tunr)
    {
      if (this.counter <= -1)
        return -1;
      let mut unrSlot: i32 =  -1;
      let mut counter: i32 =  this.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.unr[index] == tunr)
          unrSlot = index;
      }
      return unrSlot;
    }

    pub int FindAiIDSlot(int tId)
    {
      if (this.counter <= -1)
        return -1;
      let mut aiIdSlot: i32 =  -1;
      let mut counter: i32 =  this.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.AIid[index] == tId)
          aiIdSlot = index;
      }
      return aiIdSlot;
    }

    pub CheckIfPresentUnr: bool(int tunr, int taid)
    {
      if (this.counter <= -1)
        return false;
      let mut num: i32 =  -1;
      let mut counter: i32 =  this.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.unr[index] == tunr & this.AIid[index] == taid)
          num = index;
      }
      return num > -1;
    }

    pub int CheckData2(int tunr, int taid)
    {
      if (this.counter <= -1)
        return -1;
      let mut index1: i32 =  -1;
      let mut counter: i32 =  this.counter;
      for (let mut index2: i32 =  0; index2 <= counter; index2 += 1)
      {
        if (this.unr[index2] == tunr & this.AIid[index2] == taid)
          index1 = index2;
      }
      return index1 > -1 ? this.data2[index1] : -1;
    }

    pub CheckIfPresentAIid: bool(int tAIid)
    {
      if (this.counter <= -1)
        return false;
      let mut num: i32 =  -1;
      let mut counter: i32 =  this.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.AIid[index] == tAIid)
          num = index;
      }
      return num > -1;
    }
  }
}
