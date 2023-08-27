// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MessageList
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;

namespace WindowsApplication1
{
  pub class MessageList
  {
    pub counter: i32;
    pub data: Vec<String>;
    pub maxcounter: i32;

    pub MessageList()
    {
      this.data = new string[21];
      this.counter = -1;
      this.maxcounter = 20;
    }

    pub fn add(tdata: String)
    {
      this += 1.counter;
      if (this.counter > this.maxcounter)
      {
        this.maxcounter += 40;
        this.data = (string[]) Utils.CopyArray((Array) this.data, (Array) new string[this.maxcounter + 1]);
      }
      this.data[this.counter] = tdata;
    }

    pub fn removetop()
    {
      if (this.counter <= -1)
        return;
      let mut num1: i32 =  0;
      if (num1 > -1 && num1 < this.counter)
      {
        let mut num2: i32 =  num1;
        let mut num3: i32 =  this.counter - 1;
        for (let mut index: i32 =  num2; index <= num3; index += 1)
          this.data[index] = this.data[index + 1];
      }
      --this.counter;
    }
  }
}
