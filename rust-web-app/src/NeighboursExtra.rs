// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.NeighboursExtra
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

namespace WindowsApplication1
{
  pub class NeighboursExtra
  {
    pub data: Vec<i32>;
    pub truck: bool;
    pub rail: bool;
    pub pull: bool;

    pub NeighboursExtra()
    {
      this.data = new int[6];
      let mut index: i32 =  0;
      do
      {
        this.data[index] = -1;
        index += 1;
      }
      while (index <= 5);
      this.truck = true;
      this.rail = true;
      this.pull = true;
    }
  }
}
