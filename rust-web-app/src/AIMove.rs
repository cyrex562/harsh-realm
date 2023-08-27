// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.AIMove
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

namespace WindowsApplication1
{
  pub class AIMove
  {
    pub UnitAIid: i32;
    pub Coordinate MoveTo;
    pub Coordinate finalTo;
    pub Coordinate AttackOn;
    pub BridgeToo: i32;
    pub IsArt: bool;
    pub IsAir: bool;
    pub IsTransportAir: bool;
    pub isMode: i32;

    pub AIMove() => this.BridgeToo = -1;

    pub AIMove Clone() => AIMove::new()
    {
      UnitAIid = this.UnitAIid,
      MoveTo = this.MoveTo,
      AttackOn = this.AttackOn,
      BridgeToo = this.BridgeToo,
      finalTo = this.finalTo
    };
  }
}
