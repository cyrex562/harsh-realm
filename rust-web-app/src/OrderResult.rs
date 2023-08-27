// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.OrderResult
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

namespace WindowsApplication1
{
  pub class OrderResult
  {
    pub OK: bool;
    pub ErrorString: String;
    pub InfoString: String;
    pub Data: i32;
    pub CoordList CList;
    pub BattleUnit: i32;
    pub BattleX: i32;
    pub BattleY: i32;
    pub BattleMap: i32;
    pub BattleIntercept: bool;

    pub OrderResult()
    {
      this.BattleIntercept = false;
      this.CList = CoordList::new();
      this.BattleUnit = -1;
      this.BattleX = -1;
      this.BattleY = -1;
      this.BattleIntercept = false;
    }
  }
}
