// Decompiled with JetBrains decompiler
#![allow(non_snake_case)]
use rand::random;
use rand::seq::IteratorRandom;

// Type: WindowsApplication1.SimpleList
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.IO;
// usingSystem.Runtime.Serialization;
// usingSystem.Runtime.Serialization.Formatters.Binary;

// namespace WindowsApplication1
// {
//   [Serializable]
//   pub class SimpleList : ISerializable
#[derive(Default,Clone)]
pub struct SimpleList {
    pub Counter: isize,
    pub MaxCounter: i32,
    pub Id: Vec<i32>,
    pub Weight: Vec<i32>,
    pub Data1: Vec<i32>,
    pub Data2: Vec<i32>,
    pub Data3: Vec<i32>,
    pub Data4: Vec<i32>,
    pub Data5: Vec<i32>,

}
// }

impl SimpleList {
    // pub SimpleList()
    pub fn new() -> Self
    {
        Self {
            Id: vec![],
            Weight: vec![],
            Data1: vec![],
            Data2: vec![],
            Data3: vec![],
            Data4: vec![],
            Data5: vec![],
            Counter: -1,
            MaxCounter: 10
        }

    }

    // pub SimpleList Clone()
    // {
    //   if (self.Counter == -1)
    //     return SimpleList::new();
    //   BinaryFormatter binaryFormatter = BinaryFormatter::new();
    //   MemoryStream serializationStream = MemoryStream::new();
    //   binaryFormatter.Serialize((Stream) serializationStream,  this);
    //   serializationStream.Position = 0L;
    //   return (SimpleList) binaryFormatter.Deserialize((Stream) serializationStream);
    // }

    // pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    // {
    //   info.AddValue("Counter", self.Counter);
    //   info.AddValue("MaxCounter", self.MaxCounter);
    //   info.AddValue("Id",  self.Id);
    //   info.AddValue("Weight",  self.Weight);
    //   info.AddValue("Data1",  self.Data1);
    //   info.AddValue("Data2",  self.Data2);
    //   info.AddValue("Data3",  self.Data3);
    //   info.AddValue("Data4",  self.Data4);
    //   info.AddValue("Data5",  self.Data5);
    // }

    // pub fn make_simple_list(info: &SerializationInfo, context: &StreamingContext) -> Self
    // pub fn make_simple_list(context: &StreamingContext) -> Self
    // {
    //   // self.Id = new int[11];
    //   // self.Weight = new int[11];
    //   // self.Data1 = new int[11];
    //   // self.Data2 = new int[11];
    //   // self.Data3 = new int[11];
    //   // self.Data4 = new int[11];
    //   // self.Data5 = new int[11];
    //   // self.Counter = info.GetInt32(nameof (Counter));
    //   // self.MaxCounter = info.GetInt32(nameof (MaxCounter));
    //   // self.Id = new int[self.MaxCounter + 1];
    //   // self.Weight = new int[self.MaxCounter + 1];
    //   // self.Data1 = new int[self.MaxCounter + 1];
    //   // self.Data2 = new int[self.MaxCounter + 1];
    //   // self.Data3 = new int[self.MaxCounter + 1];
    //   // self.Data4 = new int[self.MaxCounter + 1];
    //   // self.Data5 = new int[self.MaxCounter + 1];
    //   // self.Id = (int[]) info.GetValue(nameof (Id), self.Id.GetType());
    //   // self.Weight = (int[]) info.GetValue(nameof (Weight), self.Weight.GetType());
    //   // self.Data1 = (int[]) info.GetValue(nameof (Data1), self.Data1.GetType());
    //   // self.Data2 = (int[]) info.GetValue(nameof (Data2), self.Data2.GetType());
    //   // self.Data3 = (int[]) info.GetValue(nameof (Data3), self.Data3.GetType());
    //   // self.Data4 = (int[]) info.GetValue(nameof (Data4), self.Data4.GetType());
    //   // self.Data5 = (int[]) info.GetValue(nameof (Data5), self.Data5.GetType());
    //     todo!()
    // }

    // pub fn Remove(tid: i32)
    pub fn Remove(&mut self, tid: i32)
    {
      let mut nr: isize = self.FindNr(tid);
      if nr == -1 {
          return;
      }
      if nr < self.Counter
      {
        let mut num1: isize = nr;
        let mut num2: isize = self.Counter - 1;
        // for (let mut index: i32 = num1; index <= num2; index += 1)
        for index in num1 as usize .. num2 as usize
          {
          self.Id[index] = self.Id[index + 1];
          self.Weight[index] = self.Weight[index + 1];
          self.Data1[index] = self.Data1[index + 1];
          self.Data2[index] = self.Data2[index + 1];
          self.Data3[index] = self.Data3[index + 1];
          self.Data4[index] = self.Data4[index + 1];
          self.Data5[index] = self.Data5[index + 1];
        }
      }
      self.Counter -= 1;
    }

    pub fn ReverseWeights(&mut self)
    {
      let mut num1: i32 = 999999;
      let mut counter1 = self.Counter as usize;
      let mut num2 = 0i32;
      // for (let mut index: i32 = 0; index <= counter1; index += 1)
      for index in 0usize .. counter1
        {
        if self.Weight[index] > num2 {
            num2 = self.Weight[index];
        }
        if num1 > self.Weight[index] {
            num1 = self.Weight[index];
        }
      }
      let mut counter2 = self.Counter as usize;
      // for (let mut index: i32 = 0; index <= counter2; index += 1)
      for index in 0usize .. counter2
        {
        self.Weight[index] = num2 - self.Weight[index];
        if num1 > self.Weight[index] {
            self.Weight[index] = num1;
        }
      }
    }

    // pub fn removeWeight0orLower()
    pub fn removeWeidth0orLower(&mut self) {
        // for (let mut counter: i32 = self.Counter; counter >= 0; counter += -1)
        for counter in self.Counter as usize..0 {
            if self.Weight[counter] <= 0 {
                self.RemoveNr(counter);
            }
        }
    }

    // pub fn GetRandomIdbasedOnWeight() -> i32
    pub fn GetRandomIdbaseOnWeight(&mut self) -> i32
    {
      let mut counter1 = self.Counter as usize;
      // num1: i32;
      let mut num1 = 0i32;
        // for (let mut index: i32 = 0; index <= counter1; index += 1)
        for index in 0 .. counter1
        {
            num1 += self.Weight[index];
        }
      if num1 < 1 {
          return -1;
      }
      // let mut num2: i32 =  Math.Round( (1f + Conversion.Int(VBMath.Rnd() *  num1)));
      let mut num2 = 1f32 + (rand::random::<f32>() * num1);
        let mut num3 = 0f32;
      let mut counter2: i32 = self.Counter as i32;
      // for (let mut index: i32 = 0; index <= counter2; index += 1)
      for index in 0 as usize .. counter2 as usize
        {
        num3 += self.Weight[index];
        if num2 <= num3 {
            return self.Id[index];
        }
      }
      return -1;
    }

    // pub fn GetRandomIdbasedOnWeightWithSeed(object tseed) -> i32
    pub fn GetRandomIdbasedOnWeightWithSeed(&mut self, tseed: i32) -> i32 {
        // Random random = new Random(Conversions.ToInteger(tseed));
        // let random = rand::random::<i32>();
        let mut rng = rand::thread_rng();
        let range_of_items: Vec<i32> = (1..1000).collect();
        let mut counter1 = self.Counter;
        // num1: i32;
        let mut num1 = 0;
        // for (let mut index: i32 = 0; index <= counter1; index += 1)
        for index in 0..counter1 as usize {
            num1 += self.Weight[index];
        }
        let mut num2 = 0;
        if num1 < 1 {
            let mut num3 = self.Counter + 1;
            // let mut num4: i32 =  Math.Round(1.0 + Conversion.Int( random.Next(0, 1000) / 1000.0 *  num3));

            let rand_val = range_of_items.iter().choose(&mut rng).unwrap();
            let mut num4 = f32::round(1.0 + rand_val / 1000.0 * num3);
            let mut num5: i32 = 0;
            num2 = -1;
            let mut counter2 = self.Counter;
            // for (let mut index: i32 = 0; index <= counter2; index += 1)
            for index in 0..counter2 as usize {
                num5 += 1;
                if num4 <= num5 as f32 {
                    return self.Id[index];
                }
            }
            return -1;
        }
        // let mut num6: i32 = Math.Round(1.0 + Conversion.Int(random.Next(0, 1000) / 1000.0 * num1));
        let next_val = range_of_items.iter().choose(&mut rng).unwrap();
        let mut num6 = f32::round(1.0 + (*next_val as f32 / 1000.0) * num1 as f32);
        let mut num7 = 0f32;
        num2 = -1;
        let mut counter3 = self.Counter;
        // for (let mut index: i32 = 0; index < = counter3; index += 1)
        for index in 0..counter3 as usize {
            num7 += self.Weight[index] as f32;
            if num6 <= num7 {
                return self.Id[index];
            }
        }
        return -1;
    }

    // pub fn GetRandomSlotbasedOnWeightWithSeed(object tseed) -> i32
    pub fn GetRandomSlotBasedOnWeightWithSeed(&mut self) -> i32
    {
        let mut rng = rand::thread_rng();
        let range_of_items: Vec<i32> = (1..1000).collect();
      // Random random = new Random(Conversions.ToInteger(tseed));
      let mut counter1 = self.Counter;
      let mut num1 = 0i32;
      for index in 0 .. counter1 as usize
      {
          num1 += self.Weight[index];
      }
      let mut num2 = 0i32;
      if (self.Counter > -1isize) & (num1 as isize == 0isize)
      {
          let next_val = range_of_items.iter().choose(&mut rng).unwrap();
        let mut num3 =  f32::round(1.0 +( *next_val as f32 / 1000.0 * 1.0));
        let mut num4: i32 = 0;
        num2 = -1;
        let mut counter2 = self.Counter;
        // for (let mut onWeightWithSeed: i32 = 0; onWeightWithSeed <= counter2; onWeightWithSeed += 1)
        for onWeightWithSeed in 0 .. counter2 as usize
          {
          num4 += 1;
          if num3 <= 1f32 {
              return onWeightWithSeed as i32;
          }
        }
        return self.Counter as i32;
      }
      if (num1 < 1) {
          return -1;
      }
      // let mut num5: i32 =  Math.Round(1.0 + Conversion.Int( random.Next(0, 1000) / 1000.0 *  num1));
      let next_val = range_of_items.iter().choose(&mut rng).unwrap().to_owned();
        let num5 = f32::round(1.0 + (next_val / 1000.0) * num1);
        let mut num6 = 0;
      num2 = -1;
      let mut counter3 = self.Counter;
      // for (let mut onWeightWithSeed: i32 = 0; onWeightWithSeed <= counter3; onWeightWithSeed += 1)
      for onWeightWithSeed in 0 .. counter3 as usize
        {
        num6 += self.Weight[onWeightWithSeed];
        if num5 as i32 <= num6 {
            return onWeightWithSeed as i32;
        }
      }
      // return self.Counter > -1 ? self.Counter : -1;
        if self.Counter > -1 {
            return self.Counter as i32
        }
        return -1;
    }

    // pub fn RemoveNr(nr: i32)
    pub fn RemoveNr(&mut self, nr: isize) {
        if nr < 0 || nr > self.Counter {
            return;
        }
        if (nr < self.Counter) {
            let mut num1 = nr;
            let mut num2 = self.Counter - 1;
            // for (let mut index: i32 = num1; index <= num2; index += 1)
            for index in num1 as usize..num2 as usize {
                self.Id[index] = self.Id[index + 1];
                self.Weight[index] = self.Weight[index + 1];
                self.Data1[index] = self.Data1[index + 1];
                self.Data2[index] = self.Data2[index + 1];
                self.Data3[index] = self.Data3[index + 1];
                self.Data4[index] = self.Data4[index + 1];
                self.Data5[index] = self.Data5[index + 1];
            }
        }
        self.Counter -= 1;
    }

    // pub fn MultiplyWeight(multi: i32)
    pub fn MultiplyWeight(&mut self, multi: isize)
    {
      let mut counter = self.Counter;
      // for (let mut index: i32 = 0; index <= counter; index += 1)
      for index in 0 .. counter as usize
        {
          self.Weight[index] = self.Weight[index] * multi as i32;
      }
    }

    // pub fn AddWeight( SimpleList SL)
    pub fn AddWeight(&mut self, SL: &mut Self)
    {
      if (Information::IsNothing( SL) || SL.Counter == -1) {
          return;
      }
      let mut counter = SL.Counter;
      // for (let mut index: i32 = 0; index <= counter; index += 1)
      for index in 0 .. counter as usize
        {
          self.AddWeight2(SL.Id[index], SL.Weight[index]);
      }
    }

    // pub fn AddWeightBlind( SimpleList SL)
    pub fn AddWeightBlind(&mut self, SL: &mut Self)
    {
      if (Information::IsNothing( SL) || SL.Counter == -1) {
          return;
      }
      let mut counter = SL.Counter;
      // for (let mut index: i32 = 0; index <= counter; index += 1)
      for index in 0 .. counter as usize
        {
          self.AddBlind(SL.Id[index], SL.Weight[index]);
      }
    }

    // pub fn RemoveWeight( SimpleList SL)
    pub fn RemoveWeight(&mut self, SL: &mut Self)
    {
      let mut counter = SL.Counter;
      // for (let mut index: i32 = 0; index <= counter; index += 1)
      for index in 0 .. counter as usize
        {
          self.RemoveWeight2(SL.Id[index], SL.Weight[index]);

      }
    }

    // pub CanRemoveWeight: bool( SimpleList SL)
    pub fn CanRemoveWeidht(&mut self, SL: &mut Self) -> bool
    {
      let mut counter = SL.Counter;
      // for (let mut index: i32 = 0; index <= counter; index += 1)
      for index in 0 .. counter as usize
        {
        if (self.FindWeight(SL.Id[index]) < SL.Weight[index]) {
            return false;
        }
      }
      return true;
    }

    // pub fn MultiplyWeight(tid: i32, float multi)
    pub fn MultiplyWeight2(&mut self, tid: i32, multi: f32)
    {
      let mut nr = self.FindNr(tid);
      if nr == -1 {
          return;
      }
      self.Weight[nr] =  (self.Weight[nr] * multi);
    }

    pub void AddWeight(
      tid: i32,
      tweight: i32,
      let mut tdata1: i32 = 0,
      let mut tdata2: i32 = 0,
      let mut tdata3: i32 = 0,
      let mut tdata4: i32 = 0,
      let mut tdata5: i32 = 0,
      bool CheckExistence = true,
      bool CheckData1Existence = false,
      bool CheckData2Existence = false)
    {
      nr: i32;
      if (CheckExistence)
      {
        nr = self.FindNr(tid);
        if (nr == -1)
        {
          CheckExistence = false;
          CheckData1Existence = false;
          CheckData2Existence = false;
        }
      }
      if (CheckData1Existence)
      {
        if (CheckExistence)
        {
          nr = self.FindNr(tid, tdata1);
          if (nr == -1)
          {
            CheckExistence = false;
            CheckData1Existence = false;
            CheckData2Existence = false;
          }
        }
        else
        {
          nr = self.FindNr(-1, tdata1);
          if (nr == -1)
          {
            CheckExistence = false;
            CheckData1Existence = false;
            CheckData2Existence = false;
          }
        }
      }
      if (CheckData2Existence)
      {
        if (CheckExistence & CheckData1Existence)
        {
          nr = self.FindNr(tid, tdata1, tdata2);
          if (nr == -1)
          {
            CheckExistence = false;
            CheckData1Existence = false;
            CheckData2Existence = false;
          }
        }
        else
        {
          nr = self.FindNr(-1, tdata2: tdata2);
          if (nr == -1)
          {
            CheckExistence = false;
            CheckData1Existence = false;
            CheckData2Existence = false;
          }
        }
      }
      if (CheckExistence | CheckData1Existence | CheckData2Existence)
      {
        self.Id[nr] = tid;
        int[] weight = self.Weight;
        int[] numArray = weight;
        let mut index1: i32 = nr;
        let mut index2: i32 = index1;
        let mut num: i32 = weight[index1] + tweight;
        numArray[index2] = num;
        if (tdata1 > 0)
          self.Data1[nr] = tdata1;
        if (tdata2 > 0)
          self.Data2[nr] = tdata2;
        if (tdata3 > 0)
          self.Data3[nr] = tdata3;
        if (tdata4 > 0)
          self.Data4[nr] = tdata4;
        if (tdata5 <= 0)
          return;
        self.Data5[nr] = tdata5;
      }
      else
      {
        this += 1.Counter;
        if (self.Counter > self.MaxCounter)
        {
          self.MaxCounter += 100;
          self.Id = (int[]) Utils.CopyArray((Array) self.Id, (Array) new int[self.MaxCounter + 1]);
          self.Weight = (int[]) Utils.CopyArray((Array) self.Weight, (Array) new int[self.MaxCounter + 1]);
          self.Data1 = (int[]) Utils.CopyArray((Array) self.Data1, (Array) new int[self.MaxCounter + 1]);
          self.Data2 = (int[]) Utils.CopyArray((Array) self.Data2, (Array) new int[self.MaxCounter + 1]);
          self.Data3 = (int[]) Utils.CopyArray((Array) self.Data3, (Array) new int[self.MaxCounter + 1]);
          self.Data4 = (int[]) Utils.CopyArray((Array) self.Data4, (Array) new int[self.MaxCounter + 1]);
          self.Data5 = (int[]) Utils.CopyArray((Array) self.Data5, (Array) new int[self.MaxCounter + 1]);
        }
        self.Id[self.Counter] = tid;
        self.Weight[self.Counter] = tweight;
        self.Data1[self.Counter] = tdata1;
        self.Data2[self.Counter] = tdata2;
        self.Data3[self.Counter] = tdata3;
        self.Data4[self.Counter] = tdata4;
        self.Data5[self.Counter] = tdata5;
      }
    }

    pub fn AddData(tid: i32, tdataNr: i32, tvalue: i32)
    {
      let mut nr: i32 = self.FindNr(tid);
      if (nr > -1)
      {
        self.Id[nr] = tid;
        if (tdataNr == 1)
        {
          int[] data1 = self.Data1;
          int[] numArray = data1;
          let mut index1: i32 = nr;
          let mut index2: i32 = index1;
          let mut num: i32 = data1[index1] + tvalue;
          numArray[index2] = num;
        }
        if (tdataNr == 2)
        {
          int[] data2 = self.Data2;
          int[] numArray = data2;
          let mut index3: i32 = nr;
          let mut index4: i32 = index3;
          let mut num: i32 = data2[index3] + tvalue;
          numArray[index4] = num;
        }
        if (tdataNr == 3)
        {
          int[] data3 = self.Data3;
          int[] numArray = data3;
          let mut index5: i32 = nr;
          let mut index6: i32 = index5;
          let mut num: i32 = data3[index5] + tvalue;
          numArray[index6] = num;
        }
        if (tdataNr == 4)
        {
          int[] data4 = self.Data4;
          int[] numArray = data4;
          let mut index7: i32 = nr;
          let mut index8: i32 = index7;
          let mut num: i32 = data4[index7] + tvalue;
          numArray[index8] = num;
        }
        if (tdataNr != 5)
          return;
        int[] data5 = self.Data5;
        int[] numArray1 = data5;
        let mut index9: i32 = nr;
        let mut index10: i32 = index9;
        let mut num1: i32 = data5[index9] + tvalue;
        numArray1[index10] = num1;
      }
      else
      {
        this += 1.Counter;
        if (self.Counter > self.MaxCounter)
        {
          self.MaxCounter += 100;
          self.Id = (int[]) Utils.CopyArray((Array) self.Id, (Array) new int[self.MaxCounter + 1]);
          self.Weight = (int[]) Utils.CopyArray((Array) self.Weight, (Array) new int[self.MaxCounter + 1]);
          self.Data1 = (int[]) Utils.CopyArray((Array) self.Data1, (Array) new int[self.MaxCounter + 1]);
          self.Data2 = (int[]) Utils.CopyArray((Array) self.Data2, (Array) new int[self.MaxCounter + 1]);
          self.Data3 = (int[]) Utils.CopyArray((Array) self.Data3, (Array) new int[self.MaxCounter + 1]);
          self.Data4 = (int[]) Utils.CopyArray((Array) self.Data4, (Array) new int[self.MaxCounter + 1]);
          self.Data5 = (int[]) Utils.CopyArray((Array) self.Data5, (Array) new int[self.MaxCounter + 1]);
        }
        self.Id[self.Counter] = tid;
        if (tdataNr == 1)
        {
          int[] data1 = self.Data1;
          int[] numArray = data1;
          let mut counter: i32 = self.Counter;
          let mut index: i32 = counter;
          let mut num: i32 = data1[counter] + tvalue;
          numArray[index] = num;
        }
        if (tdataNr == 2)
        {
          int[] data2 = self.Data2;
          int[] numArray = data2;
          let mut counter: i32 = self.Counter;
          let mut index: i32 = counter;
          let mut num: i32 = data2[counter] + tvalue;
          numArray[index] = num;
        }
        if (tdataNr == 3)
        {
          int[] data3 = self.Data3;
          int[] numArray = data3;
          let mut counter: i32 = self.Counter;
          let mut index: i32 = counter;
          let mut num: i32 = data3[counter] + tvalue;
          numArray[index] = num;
        }
        if (tdataNr == 4)
        {
          int[] data4 = self.Data4;
          int[] numArray = data4;
          let mut counter: i32 = self.Counter;
          let mut index: i32 = counter;
          let mut num: i32 = data4[counter] + tvalue;
          numArray[index] = num;
        }
        if (tdataNr != 5)
          return;
        int[] data5 = self.Data5;
        int[] numArray2 = data5;
        let mut counter1: i32 = self.Counter;
        let mut index11: i32 = counter1;
        let mut num2: i32 = data5[counter1] + tvalue;
        numArray2[index11] = num2;
      }
    }

    pub void RemoveWeight(
      tid: i32,
      tweight: i32,
      let mut tdata1: i32 = 0,
      let mut tdata2: i32 = 0,
      let mut tdata3: i32 = 0,
      let mut tdata4: i32 = 0,
      let mut tdata5: i32 = 0,
      bool CheckExistence = true,
      bool CheckData1Existence = false)
    {
      nr: i32;
      if (CheckExistence)
      {
        nr = self.FindNr(tid);
        if (nr == -1)
          CheckExistence = false;
      }
      if (CheckData1Existence)
      {
        nr = self.FindNr(-1, tdata1);
        if (nr == -1)
          CheckData1Existence = false;
      }
      if (CheckExistence | CheckData1Existence)
      {
        self.Id[nr] = tid;
        int[] weight = self.Weight;
        int[] numArray = weight;
        let mut index1: i32 = nr;
        let mut index2: i32 = index1;
        let mut num: i32 = weight[index1] - tweight;
        numArray[index2] = num;
        self.Data1[nr] = tdata1;
        self.Data2[nr] = tdata2;
        self.Data3[nr] = tdata3;
        self.Data4[nr] = tdata4;
        self.Data5[nr] = tdata5;
      }
      else
      {
        this += 1.Counter;
        if (self.Counter > self.MaxCounter)
        {
          self.MaxCounter += 100;
          self.Id = (int[]) Utils.CopyArray((Array) self.Id, (Array) new int[self.MaxCounter + 1]);
          self.Weight = (int[]) Utils.CopyArray((Array) self.Weight, (Array) new int[self.MaxCounter + 1]);
          self.Data1 = (int[]) Utils.CopyArray((Array) self.Data1, (Array) new int[self.MaxCounter + 1]);
          self.Data2 = (int[]) Utils.CopyArray((Array) self.Data2, (Array) new int[self.MaxCounter + 1]);
          self.Data3 = (int[]) Utils.CopyArray((Array) self.Data3, (Array) new int[self.MaxCounter + 1]);
          self.Data4 = (int[]) Utils.CopyArray((Array) self.Data4, (Array) new int[self.MaxCounter + 1]);
          self.Data5 = (int[]) Utils.CopyArray((Array) self.Data5, (Array) new int[self.MaxCounter + 1]);
          self.Weight[self.Counter] = 0;
        }
        else
          self.Weight[self.Counter] = 0;
        self.Id[self.Counter] = tid;
        int[] weight = self.Weight;
        int[] numArray = weight;
        let mut counter: i32 = self.Counter;
        let mut index: i32 = counter;
        let mut num: i32 = weight[counter] - tweight;
        numArray[index] = num;
        self.Data1[self.Counter] = tdata1;
        self.Data2[self.Counter] = tdata2;
        self.Data3[self.Counter] = tdata3;
        self.Data4[self.Counter] = tdata4;
        self.Data5[self.Counter] = tdata5;
      }
    }

    pub void Add(
      tid: i32,
      tweight: i32,
      let mut tdata1: i32 = 0,
      let mut tdata2: i32 = 0,
      let mut tdata3: i32 = 0,
      let mut tdata4: i32 = 0,
      let mut tdata5: i32 = 0,
      bool CheckExistence = true,
      bool CheckData1Existence = false)
    {
      nr: i32;
      if (CheckExistence)
      {
        nr = self.FindNr(tid);
        if (nr == -1)
          CheckExistence = false;
      }
      if (CheckData1Existence)
      {
        nr = self.FindNr(-1, tdata1);
        if (nr == -1)
        {
          CheckData1Existence = false;
          CheckExistence = false;
        }
      }
      if (CheckExistence | CheckData1Existence)
      {
        self.Id[nr] = tid;
        self.Weight[nr] = tweight;
        self.Data1[nr] = tdata1;
        self.Data2[nr] = tdata2;
        self.Data3[nr] = tdata3;
        self.Data4[nr] = tdata4;
        self.Data5[nr] = tdata4;
      }
      else
      {
        this += 1.Counter;
        if (self.Counter > self.MaxCounter)
        {
          self.MaxCounter += 100;
          self.Id = (int[]) Utils.CopyArray((Array) self.Id, (Array) new int[self.MaxCounter + 1]);
          self.Weight = (int[]) Utils.CopyArray((Array) self.Weight, (Array) new int[self.MaxCounter + 1]);
          self.Data1 = (int[]) Utils.CopyArray((Array) self.Data1, (Array) new int[self.MaxCounter + 1]);
          self.Data2 = (int[]) Utils.CopyArray((Array) self.Data2, (Array) new int[self.MaxCounter + 1]);
          self.Data3 = (int[]) Utils.CopyArray((Array) self.Data3, (Array) new int[self.MaxCounter + 1]);
          self.Data4 = (int[]) Utils.CopyArray((Array) self.Data4, (Array) new int[self.MaxCounter + 1]);
          self.Data5 = (int[]) Utils.CopyArray((Array) self.Data5, (Array) new int[self.MaxCounter + 1]);
        }
        self.Id[self.Counter] = tid;
        self.Weight[self.Counter] = tweight;
        self.Data1[self.Counter] = tdata1;
        self.Data2[self.Counter] = tdata2;
        self.Data3[self.Counter] = tdata3;
        self.Data4[self.Counter] = tdata4;
        self.Data5[self.Counter] = tdata5;
      }
    }

    pub fn AddBlind(tid: i32, tweight: i32)
    {
      this += 1.Counter;
      if (self.Counter > self.MaxCounter)
      {
        self.MaxCounter += 100;
        self.Id = (int[]) Utils.CopyArray((Array) self.Id, (Array) new int[self.MaxCounter + 1]);
        self.Weight = (int[]) Utils.CopyArray((Array) self.Weight, (Array) new int[self.MaxCounter + 1]);
      }
      self.Id[self.Counter] = tid;
      self.Weight[self.Counter] = tweight;
    }

    pub fn FindNr(tid: i32, let mut tdata1: i32 = -1, let mut tdata2: i32 = -1, let mut tdata3: i32 = -1, let mut tdata4: i32 = -1, let mut tWeight: i32 = -1) -> i32
    {
      if (self.Counter < 0)
        return -1;
      let mut counter: i32 = self.Counter;
      for (let mut nr: i32 = 0; nr <= counter; nr += 1)
      {
        let mut num1: i32 = 0;
        let mut num2: i32 = 0;
        let mut num3: i32 = 0;
        let mut num4: i32 = 0;
        let mut num5: i32 = 0;
        let mut num6: i32 = 0;
        if (self.Id[nr] == tid | tid == -1)
          num2 = 1;
        if (self.Weight[nr] == tWeight | tWeight == -1)
          num6 = 1;
        if (self.Data1[nr] == tdata1 | tdata1 == -1)
          num1 = 1;
        if (self.Data2[nr] == tdata2 | tdata2 == -1)
          num3 = 1;
        if (self.Data3[nr] == tdata3 | tdata3 == -1)
          num4 = 1;
        if (self.Data4[nr] == tdata4 | tdata4 == -1)
          num5 = 1;
        if (num2 == 1 & num1 == 1 & num3 == 1 & num4 == 1 & num5 == 1 & num6 == 1)
          return nr;
      }
      return -1;
    }

    pub fn FindWeight(tid: i32, let mut tdata1: i32 = -1, let mut tdata2: i32 = -1, let mut tdata3: i32 = -1, let mut tdata4: i32 = -1, let mut tWeight: i32 = -1) -> i32
    {
      let mut nr: i32 = self.FindNr(tid, tdata1, tdata2, tdata3, tdata4);
      return nr > -1 ? self.Weight[nr] : 0;
    }

    pub fn totalWeight() -> i32
    {
      if (self.Counter < 0)
        return 0;
      let mut num: i32 = 0;
      let mut counter: i32 = self.Counter;
      for (let mut index: i32 = 0; index <= counter; index += 1)
        num += self.Weight[index];
      return num;
    }

    pub fn FindData(tid: i32, tDataSlot: i32) -> i32
    {
      let mut nr: i32 = self.FindNr(tid);
      if (nr <= -1)
        return 0;
      switch (tDataSlot)
      {
        case 1:
          return self.Data1[nr];
        case 2:
          return self.Data2[nr];
        case 3:
          return self.Data3[nr];
        case 4:
          return self.Data4[nr];
        case 5:
          return self.Data5[nr];
        default:
          data: i32;
          return data;
      }
    }

    pub fn Percentify()
    {
      let mut num1: i32 = 0;
      let mut num2: i32 = 0;
      let mut counter1: i32 = self.Counter;
      for (let mut index: i32 = 0; index <= counter1; index += 1)
      {
        num1 += self.Weight[index];
        if (self.Weight[index] > num2)
          num2 = self.Weight[index];
      }
      let mut counter2: i32 = self.Counter;
      for (let mut index: i32 = 0; index <= counter2; index += 1)
      {
        let mut num3: i32 = 0;
        if (num2 > 0)
          num3 =  Math.Round(Math.Ceiling( (100 * self.Weight[index]) /  num2));
        self.Weight[index] = num3;
      }
    }

    pub fn Sort()
    {
      if (self.Counter < 1)
        return;
      let mut num1: i32 = self.Counter - 1;
      for (let mut index1: i32 = 0; index1 <= num1; index1 += 1)
      {
        let mut num2: i32 = 0;
        let mut num3: i32 = self.Counter - 1;
        for (let mut index2: i32 = 0; index2 <= num3; index2 += 1)
        {
          if (self.Weight[index2] > self.Weight[index2 + 1])
          {
            num2 = 1;
            let mut num4: i32 = self.Weight[index2 + 1];
            let mut num5: i32 = self.Id[index2 + 1];
            let mut num6: i32 = self.Data1[index2 + 1];
            let mut num7: i32 = self.Data2[index2 + 1];
            let mut num8: i32 = self.Data3[index2 + 1];
            let mut num9: i32 = self.Data4[index2 + 1];
            let mut num10: i32 = self.Data5[index2 + 1];
            self.Weight[index2 + 1] = self.Weight[index2];
            self.Id[index2 + 1] = self.Id[index2];
            self.Data1[index2 + 1] = self.Data1[index2];
            self.Data2[index2 + 1] = self.Data2[index2];
            self.Data3[index2 + 1] = self.Data3[index2];
            self.Data4[index2 + 1] = self.Data4[index2];
            self.Data5[index2 + 1] = self.Data5[index2];
            self.Weight[index2] = num4;
            self.Id[index2] = num5;
            self.Data1[index2] = num6;
            self.Data2[index2] = num7;
            self.Data3[index2] = num8;
            self.Data4[index2] = num9;
            self.Data5[index2] = num10;
          }
        }
        if (num2 == 0)
          break;
      }
    }

    pub fn ReverseSort()
    {
      if (self.Counter < 1)
        return;
      let mut num1: i32 = self.Counter - 1;
      for (let mut index1: i32 = 0; index1 <= num1; index1 += 1)
      {
        let mut num2: i32 = 0;
        let mut num3: i32 = self.Counter - 1;
        for (let mut index2: i32 = 0; index2 <= num3; index2 += 1)
        {
          if (self.Weight[index2] < self.Weight[index2 + 1])
          {
            num2 = 1;
            let mut num4: i32 = self.Weight[index2 + 1];
            let mut num5: i32 = self.Id[index2 + 1];
            let mut num6: i32 = self.Data1[index2 + 1];
            let mut num7: i32 = self.Data2[index2 + 1];
            let mut num8: i32 = self.Data3[index2 + 1];
            let mut num9: i32 = self.Data4[index2 + 1];
            let mut num10: i32 = self.Data5[index2 + 1];
            self.Weight[index2 + 1] = self.Weight[index2];
            self.Id[index2 + 1] = self.Id[index2];
            self.Data1[index2 + 1] = self.Data1[index2];
            self.Data2[index2 + 1] = self.Data2[index2];
            self.Data3[index2 + 1] = self.Data3[index2];
            self.Data4[index2 + 1] = self.Data4[index2];
            self.Data5[index2 + 1] = self.Data5[index2];
            self.Weight[index2] = num4;
            self.Id[index2] = num5;
            self.Data1[index2] = num6;
            self.Data2[index2] = num7;
            self.Data3[index2] = num8;
            self.Data4[index2] = num9;
            self.Data5[index2] = num10;
          }
        }
        if (num2 == 0)
          break;
      }
    }

    pub fn SortHighSpeed() => self.ReverseSortHighSpeed(true);

    pub fn ReverseSortHighSpeed(bool normalSort = false)
    {
      let mut num1: i32 = 9999999;
      let mut num2: i32 = -9999999;
      let mut counter1: i32 = self.Counter;
      for (let mut index: i32 = 0; index <= counter1; index += 1)
      {
        if (self.Weight[index] > num2)
          num2 = self.Weight[index];
        if (self.Weight[index] < num1)
          num1 = self.Weight[index];
      }
      let mut num3: i32 = num2 - num1;
      if (num3 < 1)
        return;
      if (num3 > 25000)
      {
        if (normalSort)
          self.Sort();
        else
          self.ReverseSort();
      }
      else
      {
        int[][] numArray1 = new int[num3 + 1 + 1][];
        let mut counter2: i32 = self.Counter;
        for (let mut index1: i32 = 0; index1 <= counter2; index1 += 1)
        {
          if (Information.IsNothing( numArray1[self.Weight[index1] - num1]))
          {
            numArray1[self.Weight[index1] - num1] = new int[1];
          }
          else
          {
            int[][] numArray2 = numArray1;
            int[][] numArray3 = numArray2;
            let mut index2: i32 = self.Weight[index1] - num1;
            let mut index3: i32 = index2;
            int[] numArray4 = (int[]) Utils.CopyArray((Array) numArray3[index3], (Array) new int[numArray1[self.Weight[index1] - num1].GetUpperBound(0) + 1 + 1]);
            numArray2[index2] = numArray4;
          }
          numArray1[self.Weight[index1] - num1][numArray1[self.Weight[index1] - num1].GetUpperBound(0)] = index1;
        }
        int[] numArray5 = new int[self.Counter + 1 + 1];
        int[] numArray6 = new int[self.Counter + 1 + 1];
        int[] numArray7 = new int[self.Counter + 1 + 1];
        int[] numArray8 = new int[self.Counter + 1 + 1];
        int[] numArray9 = new int[self.Counter + 1 + 1];
        int[] numArray10 = new int[self.Counter + 1 + 1];
        int[] numArray11 = new int[self.Counter + 1 + 1];
        let mut index4: i32 = -1;
        if (normalSort)
        {
          let mut num4: i32 = num3;
          for (let mut index5: i32 = 0; index5 <= num4; index5 += 1)
          {
            if (!Information.IsNothing( numArray1[index5]))
            {
              let mut upperBound: i32 = numArray1[index5].GetUpperBound(0);
              for (let mut index6: i32 = 0; index6 <= upperBound; index6 += 1)
              {
                index4 += 1;
                numArray5[index4] = self.Id[numArray1[index5][index6]];
                numArray6[index4] = self.Weight[numArray1[index5][index6]];
                numArray7[index4] = self.Data1[numArray1[index5][index6]];
                numArray8[index4] = self.Data2[numArray1[index5][index6]];
                numArray9[index4] = self.Data3[numArray1[index5][index6]];
                numArray10[index4] = self.Data4[numArray1[index5][index6]];
                numArray11[index4] = self.Data5[numArray1[index5][index6]];
              }
            }
          }
        }
        else
        {
          for (let mut index7: i32 = num3; index7 >= 0; index7 += -1)
          {
            if (!Information.IsNothing( numArray1[index7]))
            {
              let mut upperBound: i32 = numArray1[index7].GetUpperBound(0);
              for (let mut index8: i32 = 0; index8 <= upperBound; index8 += 1)
              {
                index4 += 1;
                numArray5[index4] = self.Id[numArray1[index7][index8]];
                numArray6[index4] = self.Weight[numArray1[index7][index8]];
                numArray7[index4] = self.Data1[numArray1[index7][index8]];
                numArray8[index4] = self.Data2[numArray1[index7][index8]];
                numArray9[index4] = self.Data3[numArray1[index7][index8]];
                numArray10[index4] = self.Data4[numArray1[index7][index8]];
                numArray11[index4] = self.Data5[numArray1[index7][index8]];
              }
            }
          }
        }
        let mut num5: i32 = index4;
        for (let mut index9: i32 = 0; index9 <= num5; index9 += 1)
        {
          self.Id[index9] = numArray5[index9];
          self.Weight[index9] = numArray6[index9];
          self.Data1[index9] = numArray7[index9];
          self.Data2[index9] = numArray8[index9];
          self.Data3[index9] = numArray9[index9];
          self.Data4[index9] = numArray10[index9];
          self.Data5[index9] = numArray11[index9];
        }
        self.Counter = index4;
      }
    }

    pub fn SortOnData5()
    {
      if (self.Counter < 1)
        return;
      let mut num1: i32 = self.Counter - 1;
      for (let mut index1: i32 = 0; index1 <= num1; index1 += 1)
      {
        let mut num2: i32 = self.Counter - 1;
        for (let mut index2: i32 = 0; index2 <= num2; index2 += 1)
        {
          if (self.Data5[index2] > self.Data5[index2 + 1])
          {
            let mut num3: i32 = self.Weight[index2 + 1];
            let mut num4: i32 = self.Id[index2 + 1];
            let mut num5: i32 = self.Data1[index2 + 1];
            let mut num6: i32 = self.Data2[index2 + 1];
            let mut num7: i32 = self.Data3[index2 + 1];
            let mut num8: i32 = self.Data4[index2 + 1];
            let mut num9: i32 = self.Data5[index2 + 1];
            self.Weight[index2 + 1] = self.Weight[index2];
            self.Id[index2 + 1] = self.Id[index2];
            self.Data1[index2 + 1] = self.Data1[index2];
            self.Data2[index2 + 1] = self.Data2[index2];
            self.Data3[index2 + 1] = self.Data3[index2];
            self.Data4[index2 + 1] = self.Data4[index2];
            self.Data5[index2 + 1] = self.Data5[index2];
            self.Weight[index2] = num3;
            self.Id[index2] = num4;
            self.Data1[index2] = num5;
            self.Data2[index2] = num6;
            self.Data3[index2] = num7;
            self.Data4[index2] = num8;
            self.Data5[index2] = num9;
          }
        }
      }
    }
}
