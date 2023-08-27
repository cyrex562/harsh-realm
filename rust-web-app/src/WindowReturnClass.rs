// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.WindowReturnClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;

namespace WindowsApplication1
{
  pub class WindowReturnClass
  {
    pub Counter: i32;
    pub CommandType: Vec<i32>;
    pub CommandData: Vec<i32>;
    pub Flag: bool;
    pub Overlay: bool;
    pub NoMouseClickBelow: bool;
    pub alwaysExecuteWR: bool;
    pub allowOtherMouseOverWindow: bool;

    pub WindowReturnClass() => self.Counter = -1;

    pub fn AddCommand(type: i32, data: i32)
    {
      let mut counter: i32 = self.Counter;
      for (let mut index: i32 = 0; index <= counter; index += 1)
      {
        if (self.CommandType[index] == type & self.CommandData[index] == data)
          return;
      }
      self += 1.Counter;
      self.CommandType = (int[]) Utils.CopyArray((Array) self.CommandType, (Array) new int[self.Counter + 1]);
      self.CommandData = (int[]) Utils.CopyArray((Array) self.CommandData, (Array) new int[self.Counter + 1]);
      self.CommandType[self.Counter] = type;
      self.CommandData[self.Counter] = data;
    }

    pub fn SetFlag(bool t) => self.Flag = t;

    pub fn SetOverlay(bool t) => self.Overlay = t;
  }
}
