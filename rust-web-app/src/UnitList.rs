// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UnitList
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;

namespace WindowsApplication1
{
  pub class UnitList
  {
    pub counter: i32;
    pub unr: Vec<i32>;
    pub data: Vec<i32>;
    pub data2: Vec<i32>;
    pub data3: Vec<i32>;
    pub maxcounter: i32;

    pub UnitList()
    {
      self.unr = new int[21];
      self.data = new int[21];
      self.data2 = new int[21];
      self.data3 = new int[21];
      self.counter = -1;
      self.maxcounter = 20;
    }

    pub fn add(tunr: i32, let mut tdata: i32 = -1, let mut tdata2: i32 = -1, let mut tdata3: i32 = -1)
    {
      self += 1.counter;
      if (self.counter > self.maxcounter)
      {
        self.maxcounter += 40;
        self.unr = (int[]) Utils.CopyArray((Array) self.unr, (Array) new int[self.maxcounter + 1]);
        self.data = (int[]) Utils.CopyArray((Array) self.data, (Array) new int[self.maxcounter + 1]);
        self.data2 = (int[]) Utils.CopyArray((Array) self.data2, (Array) new int[self.maxcounter + 1]);
        self.data3 = (int[]) Utils.CopyArray((Array) self.data3, (Array) new int[self.maxcounter + 1]);
      }
      self.unr[self.counter] = tunr;
      self.data[self.counter] = tdata;
      self.data2[self.counter] = tdata2;
      self.data3[self.counter] = tdata3;
    }

    pub fn SortOnData()
    {
      if (self.counter < 1)
        return;
      let mut num1: i32 = self.counter - 1;
      for (let mut index1: i32 = 0; index1 <= num1; index1 += 1)
      {
        let mut num2: i32 = 0;
        let mut num3: i32 = self.counter - 1;
        for (let mut index2: i32 = 0; index2 <= num3; index2 += 1)
        {
          if (self.data[index2] > self.data[index2 + 1])
          {
            num2 = 1;
            let mut num4: i32 = self.unr[index2 + 1];
            let mut num5: i32 = self.data[index2 + 1];
            let mut num6: i32 = self.data2[index2 + 1];
            let mut num7: i32 = self.data3[index2 + 1];
            self.unr[index2 + 1] = self.unr[index2];
            self.data[index2 + 1] = self.data[index2];
            self.data2[index2 + 1] = self.data2[index2];
            self.data3[index2 + 1] = self.data3[index2];
            self.unr[index2] = num4;
            self.data[index2] = num5;
            self.data2[index2] = num6;
            self.data3[index2] = num7;
          }
        }
        if (num2 == 0)
          break;
      }
    }

    pub fn remove(tunr: i32)
    {
      if (self.counter <= -1)
        return;
      let mut num1: i32 = -1;
      let mut counter: i32 = self.counter;
      for (let mut index: i32 = 0; index <= counter; index += 1)
      {
        if (self.unr[index] == tunr)
          num1 = index;
      }
      if (num1 > -1 && num1 < self.counter)
      {
        let mut num2: i32 = num1;
        let mut num3: i32 = self.counter - 1;
        for (let mut index: i32 = num2; index <= num3; index += 1)
        {
          self.unr[index] = self.unr[index + 1];
          self.data[index] = self.data[index + 1];
          self.data2[index] = self.data2[index + 1];
          self.data3[index] = self.data3[index + 1];
        }
      }
      --self.counter;
    }

    pub CheckIfPresent: bool(tunr: i32)
    {
      if (self.counter <= -1)
        return false;
      let mut num: i32 = -1;
      let mut counter: i32 = self.counter;
      for (let mut index: i32 = 0; index <= counter; index += 1)
      {
        if (self.unr[index] == tunr)
          num = index;
      }
      return num > -1;
    }
  }
}
