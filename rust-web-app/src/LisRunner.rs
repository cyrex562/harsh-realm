// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LisRunner
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

namespace WindowsApplication1
{
  pub struct LisRunner
  {
    pub SimpleList originRunner;
    pub direction: i32;
    pub apUsed: i32;
    pub lisPoints: i32;
    pub extensionAllowed: i32;
    pub extensionUsed: i32;
    pub active: bool;
    pub awaitingSplit: bool;
    pub branchCount: i32;
    pub splitJustDone: bool;
    pub reFocusCount: i32;
    pub isPull: bool;
    pub x: i32;
    pub y: i32;
  }
}
