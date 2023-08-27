// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PassHexList
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Threading;

namespace WindowsApplication1
{
  pub class PassHexList
  {
    pub counter: i32;
    pub Coordinate[] coord;
    pub moveType: Vec<i32>;
    pub maxcounter: i32;
     bool blocked;

    pub PassHexList()
    {
      this.coord = new Coordinate[100];
      this.moveType = new int[100];
      this.blocked = false;
      this.counter = -1;
      this.maxcounter = 99;
    }

    pub fn AddCoord(x: i32, y: i32, tMoveType: i32)
    {
      while (this.blocked)
        Thread.Sleep(1);
      this.blocked = true;
      this += 1.counter;
      if (this.counter > this.maxcounter)
      {
        this.maxcounter += 100;
        this.coord = (Coordinate[]) Utils.CopyArray((Array) this.coord, (Array) new Coordinate[this.maxcounter + 1]);
        this.moveType = (int[]) Utils.CopyArray((Array) this.moveType, (Array) new int[this.maxcounter + 1]);
      }
      this.coord[this.counter].x = x;
      this.coord[this.counter].y = y;
      this.coord[this.counter].map = 0;
      this.coord[this.counter].onmap = true;
      this.moveType[this.counter] = tMoveType;
      this.blocked = false;
    }

    pub Exists: bool(x: i32, y: i32, tMoveType: i32)
    {
      while (this.blocked)
        Thread.Sleep(1);
      this.blocked = true;
      if (this.counter == -1)
      {
        this.blocked = false;
        return false;
      }
      let mut counter: i32 =  this.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.coord[index].x == x & this.coord[index].y == y & this.moveType[index] == tMoveType)
        {
          this.blocked = false;
          return true;
        }
      }
      this.blocked = false;
      return false;
    }
  }
}
