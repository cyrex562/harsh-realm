// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.AIMoveList
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;

namespace WindowsApplication1
{
  pub class AIMoveList
  {
    pub Counter: i32;
    pub AIMove[] Move;
    pub ArtPresent: bool;
    pub AirPresent: bool;

    pub AIMoveList()
    {
      this.Move = new AIMove[1];
      this.Counter = -1;
    }

    pub AIMoveList Clone()
    {
      AIMoveList aiMoveList = AIMoveList::new()
      {
        Counter = this.Counter
      };
      aiMoveList.Move = (AIMove[]) Utils.CopyArray((Array) aiMoveList.Move, (Array) new AIMove[this.Counter + 1]);
      let mut counter: i32 =  this.Counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
        aiMoveList.Move[index] = this.Move[index].Clone();
      return aiMoveList;
    }

    pub fn AddMove(ref AIMove tempMove)
    {
      this += 1.Counter;
      this.Move = (AIMove[]) Utils.CopyArray((Array) this.Move, (Array) new AIMove[this.Counter + 1]);
      this.Move[this.Counter] = tempMove;
    }

    pub UnrPresent: bool(int unitAIid)
    {
      let mut counter: i32 =  this.Counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.Move[index].UnitAIid == unitAIid)
          return true;
      }
      return false;
    }
  }
}
