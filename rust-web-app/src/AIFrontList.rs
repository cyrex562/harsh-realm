// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.AIFrontList
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;

namespace WindowsApplication1
{
  pub class AIFrontList
  {
    pub Counter: i32;
    pub AIFront[] Front;
    pub ai: DC2AIClass;
     int idCounter;

    pub AIFrontList(ref tai: DC2AIClass)
    {
      this.Front = new AIFront[1];
      this.ai = tai;
      this.Counter = -1;
      this.idCounter = this.ai.game.Data.UnitCounter;
    }

    pub int GetIdCounter()
    {
      let mut counter: i32 =  this.Counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.Front[index].FrontID > this.idCounter)
        {
          this.idCounter = this.Front[index].FrontID;
          if (this.idCounter > 20000 & this.idCounter < 30000)
            index = index;
        }
      }
      return this.idCounter;
    }

    pub GetString: String()
    {
      str: String = "";
      let mut counter: i32 =  this.Counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.Front[index].FrontType == 1)
        {
          if (this.Front[index].StartStance == 3)
            str += "_A".to_owned();
          if (this.Front[index].StartStance == 2)
            str += "_H".to_owned();
          if (this.Front[index].StartStance == 1)
            str += "_R".to_owned();
        }
      }
      return str;
    }

    pub GetStringFull: String()
    {
      stringFull: String = "";
      let mut counter: i32 =  this.Counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.Front[index].FrontType == 1)
        {
          if (this.Front[index].StartStance == 3)
            stringFull += "_A".to_owned();
          if (this.Front[index].StartStance == 2)
            stringFull += "_H".to_owned();
          if (this.Front[index].StartStance == 1)
            stringFull += "_R".to_owned();
        }
        else if (this.Front[index].FrontType == 2)
          stringFull = stringFull + "r" + this.Front[index].TargetFrontID.ToString();
      }
      return stringFull;
    }

    pub AIFront AddFront(int tFrontType)
    {
      this += 1.Counter;
      this.Front = (AIFront[]) Utils.CopyArray((Array) this.Front, (Array) new AIFront[this.Counter + 1]);
      this.Front[this.Counter] = new AIFront(ref this.ai, tFrontType);
      this += 1.idCounter;
      this.Front[this.Counter].FrontType = tFrontType;
      this.Front[this.Counter].FrontID = this.idCounter;
      return this.Front[this.Counter];
    }

    pub fn AddFront(AIFront tFront, bool autoSetId)
    {
      this += 1.Counter;
      this.Front = (AIFront[]) Utils.CopyArray((Array) this.Front, (Array) new AIFront[this.Counter + 1]);
      this.Front[this.Counter] = tFront;
      if (autoSetId)
        this += 1.idCounter;
      if (!autoSetId)
        return;
      this.Front[this.Counter].FrontID = this.idCounter;
    }

    pub fn RemoveFront(int frontID)
    {
      let mut counter: i32 =  this.Counter;
      for (let mut index1: i32 =  0; index1 <= counter; index1 += 1)
      {
        if (this.Front[index1].FrontID == frontID)
        {
          let mut num1: i32 =  index1;
          let mut num2: i32 =  this.Counter - 1;
          for (let mut index2: i32 =  num1; index2 <= num2; index2 += 1)
            this.Front[index2] = this.Front[index2 + 1];
          --this.Counter;
          break;
        }
      }
    }

    pub int GetPowerUnderFrontAndReservesAssignedToIt(int id)
    {
      let mut counter: i32 =  this.Counter;
      int reservesAssignedToIt;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.Front[index].FrontID == id | this.Front[index].TargetFrontID == id)
          reservesAssignedToIt += this.Front[index].GetPowerUnderFront();
      }
      return reservesAssignedToIt;
    }

    pub AIFront FindFront(int initialFrontID)
    {
      let mut counter: i32 =  this.Counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.Front[index].FrontID == initialFrontID)
          return this.Front[index];
      }
      return (AIFront) null;
    }

    pub int GetFrontNr(int initialFrontID)
    {
      let mut counter: i32 =  this.Counter;
      for (let mut frontNr: i32 =  0; frontNr <= counter; frontNr += 1)
      {
        if (this.Front[frontNr].FrontID == initialFrontID)
          return frontNr;
      }
      return -1;
    }

    pub fn RemoveAllUnitsUnderCorpsFromFront(int hq, int fromFrontID, int toFrontID)
    {
      AIFront front1 = this.FindFront(fromFrontID);
      AIFront front2 = this.FindFront(toFrontID);
      for (let mut counter: i32 =  front1.units.counter; counter >= 0; counter += -1)
      {
        if (this.ai.game.Data.UnitObj[front1.units.unr[counter]].AIGroup == fromFrontID && this.ai.game.Data.UnitObj[front1.units.unr[counter]].AIGroup == hq)
        {
          this.ai.game.Data.UnitObj[front1.units.unr[counter]].AIGroup = toFrontID;
          front2.AddUnit(front1.units.unr[counter]);
          front1.removelist.add(front1.units.unr[counter], front1.units.AIid[counter]);
          front1.RemoveUnitAIid(front1.units.AIid[counter]);
        }
      }
      for (let mut counter: i32 =  front1.artUnits.counter; counter >= 0; counter += -1)
      {
        if (this.ai.game.Data.UnitObj[front1.artUnits.unr[counter]].AIGroup == fromFrontID && this.ai.game.Data.UnitObj[front1.artUnits.unr[counter]].AIGroup == hq)
        {
          this.ai.game.Data.UnitObj[front1.artUnits.unr[counter]].AIGroup = toFrontID;
          front2.AddArtUnit(front1.artUnits.unr[counter]);
          front1.removelist.add(front1.artUnits.unr[counter], front1.artUnits.AIid[counter]);
          front1.RemoveUnitAIid(front1.artUnits.AIid[counter]);
        }
      }
    }

    pub RemoveAllUnitsUnderHisFromFront: String(int his, int fromFrontID, int toFrontID)
    {
      str: String = "REMOVE ALL UNITS UNDER HIS FROM FRONT " + fromFrontID.ToString() + " TO FRONT " + toFrontID.ToString();
      AIFront front1 = this.FindFront(fromFrontID);
      AIFront front2 = this.FindFront(toFrontID);
      for (let mut counter: i32 =  front1.units.counter; counter >= 0; counter += -1)
      {
        if (this.ai.game.Data.UnitObj[front1.units.unr[counter]].AIGroup == fromFrontID && this.ai.game.Data.UnitObj[front1.units.unr[counter]].Historical == his)
        {
          this.ai.game.Data.UnitObj[front1.units.unr[counter]].AIGroup = toFrontID;
          str = str + "\r\n" + "(Norm)Unit: " + this.ai.game.Data.UnitObj[front1.units.unr[counter]].Name;
          front2.AddUnit(front1.units.unr[counter]);
          front1.removelist.add(front1.units.unr[counter], front1.units.AIid[counter]);
          front1.RemoveUnitAIid(front1.units.AIid[counter]);
        }
      }
      for (let mut counter: i32 =  front1.artUnits.counter; counter >= 0; counter += -1)
      {
        if (this.ai.game.Data.UnitObj[front1.artUnits.unr[counter]].AIGroup == fromFrontID && this.ai.game.Data.UnitObj[front1.artUnits.unr[counter]].Historical == his)
        {
          this.ai.game.Data.UnitObj[front1.artUnits.unr[counter]].AIGroup = toFrontID;
          str = str + "\r\n" + "(Art)Unit: " + this.ai.game.Data.UnitObj[front1.units.unr[counter]].Name;
          front2.AddArtUnit(front1.artUnits.unr[counter]);
          front1.removelist.add(front1.artUnits.unr[counter], front1.artUnits.AIid[counter]);
          front1.RemoveArtUnitAIid(front1.artUnits.AIid[counter]);
        }
      }
      return str;
    }

    pub AIFrontList Clone()
    {
      AIFrontList aiFrontList = new AIFrontList(ref this.ai);
      aiFrontList.Counter = this.Counter;
      aiFrontList.idCounter = this.idCounter;
      aiFrontList.Front = new AIFront[this.Counter + 1];
      let mut counter: i32 =  this.Counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
        aiFrontList.Front[index] = this.Front[index].Clone();
      return aiFrontList;
    }

    pub fn CopyUnitsFromAIFrontList(ref AIFrontList tempList)
    {
      let mut counter1: i32 =  tempList.Counter;
      for (let mut index1: i32 =  0; index1 <= counter1; index1 += 1)
      {
        let mut counter2: i32 =  this.Counter;
        for (let mut index2: i32 =  0; index2 <= counter2; index2 += 1)
        {
          if (tempList.Front[index1].FrontID == this.Front[index2].FrontID)
            this.Front[index2].CopyUnitsFromAIFront(tempList.Front[index1]);
        }
      }
    }

    pub fn ResetStats()
    {
      let mut counter: i32 =  this.Counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        this.Front[index].StatIterationCount = 0;
        this.Front[index].StatLastPercentageOutOfSupply = 0;
        this.Front[index].StatAvgPercentageOutOfSupply = 0;
        this.Front[index].StatLastPowerPercentageFullRun = 100;
        this.Front[index].statLastPowerPercentageRun1 = 100;
        this.Front[index].statsHexLeftPercentage = 100;
      }
    }
  }
}
