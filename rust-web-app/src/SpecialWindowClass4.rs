// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SpecialWindowClass4
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SpecialWindowClass4 : WindowClass
  {
     okId: i32;
     RightId: i32;
     StartTurnId: i32;
     PreviewId: i32;
     RightId2: i32;
     StartTurnId2: i32;
     PreviewId2: i32;
     ListClass ListObj;
     ListClass List2Obj;
     listId: i32;
     list2Id: i32;
     useWidth: i32;
     useHeight: i32;
     SimpleList listShq;
     SimpleList listZone;
     SimpleList listAsset;
     SimpleList listItemAsset;
     bool anyZoneWithoutSHQ;
     prevAssetId: i32;
     SimpleList IL;
     SimpleStringList ILdesc;
     SimpleList PIL;
     SimpleStringList PILdesc;
     int[] assetButton;
     assetButtonCounter: i32;
     int[] assetButtonData;
     AssetOrderNumber: i32;
     orderfeedbackString: String;
     bool onPopupRefreshReCalc;
     slotAssetPresentation: i32;
     slotPerks: i32;
     slotPreviewAssetLog: i32;
     slotHexPerk: i32;
     slotPaid: i32;
     slotHexNames: i32;
     slotLandscape: i32;
     slotAssetLog: i32;
     slotConstruction: i32;
     slotZones: i32;
     slotRegKey: i32;
     slotDetail: i32;
     slotRegReg: i32;
     slotRegZoneKeys: i32;
     slotItemType: i32;
     slotRegimes: i32;
     slotZoneKeys: i32;
     slotAssetTypes: i32;
     slotAssets: i32;
     slotCharacter: i32;
     slotPortrait: i32;
     int[] itemWeight;
     itemName: Vec<String>;
     bool previewSet;

    pub fn Dispose()
    {
      if (!self.game.EditObj.layerLisPreview & self.previewSet)
      {
        let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
        for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
          for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
          {
            let mut index3: i32 = 0;
            do
            {
              self.game.Data.MapObj[0].HexObj[index1, index2].tempPreviewLIS[index3] = 0;
              self.game.Data.MapObj[0].HexObj[index1, index2].tempPreviewAssetLIS[index3] = 0;
              index3 += 1;
            }
            while (index3 <= 8);
          }
        }
      }
      base.Dispose();
    }

    pub HandleMouseMove: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      return base.HandleMouseMove(x, y);
    }

    pub SpecialWindowClass4( tGame: GameClass, tUseWidth: i32, tUseHeight: i32)
      : base( tGame, tUseWidth, tUseHeight, 8)
    {
      self.ListObj = ListClass::new();
      self.List2Obj = ListClass::new();
      self.anyZoneWithoutSHQ = false;
      self.prevAssetId = -1;
      self.assetButton = new int[600];
      self.assetButtonCounter = -1;
      self.assetButtonData = new int[600];
      self.onPopupRefreshReCalc = false;
      self.itemWeight = new int[100];
      self.itemName = new string[100];
      self.previewSet = false;
      self.useWidth = tUseWidth;
      self.useHeight = tUseHeight;
      libName: String = "SE_Data";
      self.slotPaid = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 500, 0, 0));
      self.slotHexNames = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 382, 0, 0));
      self.slotPerks = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 381, 0, 0));
      self.slotZones = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 123, 0, 0));
      self.slotZoneKeys = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 160, 0, 0));
      self.slotAssets = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 148, 0, 0));
      self.slotAssetLog = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 241, 0, 0));
      self.slotPreviewAssetLog = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 513, 0, 0));
      self.slotAssetTypes = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 144, 0, 0));
      self.slotItemType = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 149, 0, 0));
      self.slotAssetPresentation = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 166, 0, 0));
      self.slotCharacter = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 196, 0, 0));
      self.slotPortrait = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 204, 0, 0));
      self.slotZones = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 123, 0, 0));
      self.slotZoneKeys = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 160, 0, 0));
      self.slotAssets = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 148, 0, 0));
      self.slotAssetTypes = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 144, 0, 0));
      self.slotItemType = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 149, 0, 0));
      self.slotAssetPresentation = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 166, 0, 0));
      self.slotCharacter = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 196, 0, 0));
      self.slotPortrait = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 204, 0, 0));
      self.slotRegKey = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 210, 0, 0));
      self.slotDetail = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 361, 0, 0));
      self.slotLandscape = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 362, 0, 0));
      self.slotConstruction = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 150, 0, 0));
      let mut length: i32 = self.game.Data.StringListObj[self.slotItemType].Length;
      for (let mut index1: i32 = 0; index1 <= length; index1 += 1)
      {
        let mut index2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotItemType].Data[index1, 0]));
        let mut num: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotItemType].Data[index1, 3]));
        self.itemWeight[index2] = num;
        self.itemName[index2] = self.game.Data.StringListObj[self.slotItemType].Data[index1, 1];
      }
      if (!(self.game.EditObj.se1_assetSHQ > self.game.Data.UnitCounter | self.game.EditObj.se1_assetSHQ == -1))
      {
        if (self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].Regime == self.game.Data.Turn)
        {
          if (self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].Historical > -1)
          {
            if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].Historical].Type == 8)
              self.game.EditObj.se1_assetSHQ = -1;
          }
          else
            self.game.EditObj.se1_assetSHQ = -1;
        }
        else
          self.game.EditObj.se1_assetSHQ = -1;
      }
      if (!self.game.EditObj.layerLisPreview & self.game.EditObj.se1_assetMode == 2)
      {
        self.game.ProcessingObj.LIS_SetNetwork(false, true);
        self.previewSet = true;
      }
      self.ReCalculate();
      self.dostuff();
    }

    pub fn ReCalculate()
    {
      libName: String = "SE_Data";
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 381, 0, 0));
      self.listShq = SimpleList::new();
      self.listZone = SimpleList::new();
      self.game.EventRelatedObj.ExecSuperImposeMessage("Calculating", "Hold on while we calculate the details on your Assets", 0, 0, "");
      let mut unitCounter: i32 = self.game.Data.UnitCounter;
      for (let mut tid: i32 = 0; tid <= unitCounter; tid += 1)
      {
        if (self.game.Data.UnitObj[tid].Regime == self.game.Data.Turn && self.game.Data.UnitObj[tid].PreDef == -1 && self.game.Data.UnitObj[tid].Historical > -1 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[tid].Historical].Type == 8)
        {
          self.listShq.Add(tid, 0);
          if (self.game.EditObj.se1_assetSHQ == -1)
            self.game.EditObj.se1_assetSHQ = tid;
        }
      }
      bool flag = false;
      let mut length1: i32 = self.game.Data.StringListObj[self.slotZones].Length;
      for (let mut index: i32 = 0; index <= length1; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZones].Data[index, 8])) == self.game.Data.RegimeObj[self.game.Data.Turn].id)
        {
          let mut num1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZones].Data[index, 0]));
          let mut id: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZones].Data[index, 6]));
          let mut num2: i32 = -1;
          if (id > 0)
          {
            let mut locationById: i32 = self.game.HandyFunctionsObj.GetLocationByID(id);
            if (locationById > -1)
              num2 = self.game.Data.LocObj[locationById].HQ;
          }
          let mut tweight: i32 = ( Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZoneKeys].GetData2(0, num1, 1, "pop", 2))) +  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZoneKeys].GetData2(0, num1, 1, "worker", 2)))) * 100;
          self.listZone.Add(num1, tweight, num2);
          if (num2 > -1)
            self.listShq.AddWeight(num2, tweight);
          else
            flag = true;
        }
      }
      self.listShq.ReverseSort();
      self.listZone.ReverseSort();
      self.listAsset = SimpleList::new();
      self.listItemAsset = SimpleList::new();
      let mut length2: i32 = self.game.Data.StringListObj[self.slotAssets].Length;
      for (let mut tid: i32 = 0; tid <= length2; tid += 1)
      {
        let mut num: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[tid, 0]));
        if (self.listZone.FindNr(num) > -1)
          self.listAsset.Add(tid, num);
      }
      self.PIL = SimpleList::new();
      self.PILdesc = SimpleStringList::new();
      let mut stringListById2: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 166, 0, 0));
      SimpleList simpleList1 = SimpleList::new();
      SimpleList simpleList2 = SimpleList::new();
      int[] numArray1 = new int[self.listZone.Counter + 1 + 1];
      SimpleList[] simpleListArray1 = new SimpleList[self.listZone.Counter + 1 + 1];
      SimpleList[] simpleListArray2 = new SimpleList[self.listZone.Counter + 1 + 1];
      SimpleList[] simpleListArray3 = new SimpleList[self.listZone.Counter + 1 + 1];
      SimpleList[] simpleListArray4 = new SimpleList[self.listZone.Counter + 1 + 1];
      SimpleList[] simpleListArray5 = new SimpleList[self.listZone.Counter + 1 + 1];
      SimpleList[] simpleListArray6 = new SimpleList[self.listZone.Counter + 1 + 1];
      SimpleList[] simpleListArray7 = new SimpleList[self.listZone.Counter + 1 + 1];
      SimpleList[] simpleListArray8 = new SimpleList[self.listZone.Counter + 1 + 1];
      SimpleList[] simpleListArray9 = new SimpleList[self.listZone.Counter + 1 + 1];
      SimpleList[] simpleListArray10 = new SimpleList[self.listZone.Counter + 1 + 1];
      let mut num3: i32 = 0;
      int[] numArray2 = new int[ Math.Round(Conversion.Val( self.game.Data.StringListObj[self.slotAssets].GetHighestValue(9))) + 29999 + 1];
      let mut length3: i32 = self.game.Data.StringListObj[self.slotAssets].Length;
      for (let mut index1: i32 = 0; index1 <= length3; index1 += 1)
      {
        let mut index2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index1, 9]));
        numArray2[index2] = index1;
      }
      x1: i32;
      y1: i32;
      if (self.game.EditObj.se1_assetSHQ > 0)
      {
        x1 = self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].X;
        y1 = self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].Y;
        self.game.HandyFunctionsObj.MakeMovePredictionLIS_Preview(self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].X, self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].Y, self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].Map);
        let mut counter: i32 = self.listZone.Counter;
        for (let mut index3: i32 = 0; index3 <= counter; index3 += 1)
        {
          if (self.listZone.Data1[index3] == self.game.EditObj.se1_assetSHQ)
          {
            let mut id: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZones].GetData(0, self.listZone.Id[index3], 6)));
            let mut index4: i32 = -1;
            if (id > 0)
              index4 = self.game.HandyFunctionsObj.GetLocationByID(id);
            let mut num4: i32 = 0;
            if (index4 > -1)
              num4 = self.game.ProcessingObj.LIS_GetLowestPointsOnTrajectory_PREVIEW(self.game.Data.LocObj[index4].X, self.game.Data.LocObj[index4].Y, self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].X, self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].Y, true);
            numArray1[index3] = num4;
            if (numArray1[index3] < 0)
              numArray1[index3] = 0;
          }
        }
      }
      let mut counter1: i32 = self.listZone.Counter;
      tweight1: i32;
      for (let mut index5: i32 = 0; index5 <= counter1; index5 += 1)
      {
        if (self.listZone.Data1[index5] == self.game.EditObj.se1_assetSHQ)
        {
          num3 += 1;
          simpleListArray1[index5] = SimpleList::new();
          simpleListArray2[index5] = SimpleList::new();
          simpleListArray3[index5] = SimpleList::new();
          simpleListArray4[index5] = SimpleList::new();
          simpleListArray5[index5] = SimpleList::new();
          simpleListArray6[index5] = SimpleList::new();
          simpleListArray7[index5] = SimpleList::new();
          simpleListArray8[index5] = SimpleList::new();
          simpleListArray9[index5] = SimpleList::new();
          simpleListArray10[index5] = SimpleList::new();
          let mut num5: i32 = self.listZone.Id[index5];
          let mut id: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZones].GetData(0, num5, 6)));
          let mut index6: i32 = -1;
          if (id > 0)
            index6 = self.game.HandyFunctionsObj.GetLocationByID(id);
          x2: i32;
          y2: i32;
          if (index6 > -1)
          {
            x2 = self.game.Data.LocObj[index6].X;
            y2 = self.game.Data.LocObj[index6].Y;
          }
          if (index6 > -1)
          {
            if (Information.IsNothing( self.game.Data.LocObj[index6].items))
              self.game.Data.LocObj[index6].items = ItemList::new();
            let mut counter2: i32 = self.game.Data.LocObj[index6].items.list.Counter;
            for (let mut index7: i32 = 0; index7 <= counter2; index7 += 1)
            {
              let mut tid: i32 = self.game.Data.LocObj[index6].items.list.Id[index7];
              let mut tweight2: i32 = self.game.Data.LocObj[index6].items.list.Weight[index7];
              simpleListArray1[index5].AddWeight(tid, tweight2);
              self.PIL.Add(tid, tweight2, tdata2: num5, tdata4: 1, CheckExistence: false);
            }
          }
          let mut counter3: i32 = self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].items.list.Counter;
          for (let mut index8: i32 = 0; index8 <= counter3; index8 += 1)
          {
            let mut index9: i32 = self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].items.list.Id[index8];
            let mut tweight3: i32 = self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].items.list.Weight[index8];
            if (num3 == 1)
              simpleList1.AddWeight(index9, tweight3);
            self.PIL.Add(index9, tweight3, 1, num5, tdata4: 1, CheckExistence: false);
            self.PILdesc.Add("The amount of " + self.itemName[index9] + " currently available at SHQ.", 1, 1, num5, index9, 1, CheckExistence: false);
          }
          eventRelatedObj: EventRelatedClass = self.game.EventRelatedObj;
          let mut onlyZoneId: i32 = num5;
          SimpleList simpleList3 = (SimpleList) null;
           SimpleList local1 =  simpleList3;
          SimpleList simpleList4 = (SimpleList) null;
           SimpleList local2 =  simpleList4;
          let mut num6: i32 = num3 == 1 ? 1 : 0;
          eventRelatedObj.ExecMakeAssetPresentation("SE_Data", -1, -1, onlyZoneId, "", itemsProdModList: ( local1), itemsUpkeepModList: ( local2), logNeed: true, clearLogs: (num6 != 0));
          let mut length4: i32 = self.game.Data.StringListObj[stringListById2].Length;
          for (let mut index10: i32 = 0; index10 <= length4; index10 += 1)
          {
            let mut tid: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index10, 0]));
            if (tid > 0)
            {
              let mut num7: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index10, 2]));
              let mut tweight4: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index10, 3]));
              tweight1 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index10, 4]));
              let mut tweight5: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index10, 8]));
              if (tweight4 > 0)
              {
                simpleListArray2[index5].AddWeight(tid, tweight4);
                if (tweight5 > 0)
                  simpleListArray6[index5].AddWeight(tid, tweight5);
              }
              if (tweight1 > 0)
                simpleListArray3[index5].AddWeight(tid, tweight1);
            }
          }
          simpleListArray4[index5] = simpleListArray2[index5].Clone();
          simpleListArray4[index5].RemoveWeight( simpleListArray1[index5]);
          simpleListArray4[index5].removeWeight0orLower();
          simpleListArray5[index5] = simpleListArray4[index5].Clone();
          let mut num8: i32 = 0;
          let mut counter4: i32 = simpleListArray4[index5].Counter;
          for (let mut index11: i32 = 0; index11 <= counter4; index11 += 1)
          {
            let mut num9: i32 = self.itemWeight[simpleListArray4[index5].Id[index11]] * simpleListArray4[index5].Weight[index11];
            if (x1 == x2 & y1 == y2)
              num9 = 0;
            num8 += num9;
          }
          if (num8 > numArray1[index5])
          {
            let mut counter5: i32 = simpleListArray4[index5].Counter;
            for (let mut index12: i32 = 0; index12 <= counter5; index12 += 1)
            {
              if (self.itemWeight[simpleListArray4[index5].Id[index12]] > 0 | numArray1[index5] == 0 && !(x1 == x2 & y1 == y2))
              {
                let mut num10: i32 =  Math.Round(Math.Floor( (simpleListArray4[index5].Weight[index12] * numArray1[index5]) /  num8));
                simpleListArray4[index5].Weight[index12] = num10;
              }
            }
          }
          simpleList2.AddWeight( simpleListArray4[index5]);
        }
      }
      SimpleList simpleList5 = SimpleList::new();
      let mut counter6: i32 = simpleList2.Counter;
      for (let mut index: i32 = 0; index <= counter6; index += 1)
      {
        let mut tid: i32 = simpleList2.Id[index];
        let mut num11: i32 = simpleList2.Weight[index];
        let mut weight: i32 = self.game.Data.UnitObj[self.game.EditObj.se1_assetSHQ].items.list.FindWeight(tid);
        let mut tweight6: i32 = !(weight >= num11 & num11 > 0) ? (num11 <= 0 ? 0 :  Math.Round(Math.Floor( (100 * weight) /  num11))) : 100;
        simpleList5.AddWeight(tid, tweight6);
      }
      let mut tid1: i32 = 1;
      do
      {
        if (simpleList5.FindNr(tid1) == -1)
          simpleList5.AddWeight(tid1, 100);
        tid1 += 1;
      }
      while (tid1 <= 30);
      let mut counter7: i32 = self.listZone.Counter;
      str1: String;
      str2: String;
      str3: String;
      num12: i32;
      num13: i32;
      for (let mut index13: i32 = 0; index13 <= counter7; index13 += 1)
      {
        if (self.listZone.Data1[index13] == self.game.EditObj.se1_assetSHQ)
        {
          let mut num14: i32 = self.listZone.Id[index13];
          let mut id: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZones].GetData(0, num14, 6)));
          let mut index14: i32 = -1;
          if (id > 0)
            index14 = self.game.HandyFunctionsObj.GetLocationByID(id);
          x3: i32;
          y3: i32;
          if (index14 > -1)
          {
            x3 = self.game.Data.LocObj[index14].X;
            y3 = self.game.Data.LocObj[index14].Y;
          }
          if (index14 > -1)
          {
            SimpleList simpleList6 = SimpleList::new();
            SimpleList simpleList7 = SimpleList::new();
            let mut index15: i32 = 1;
            do
            {
              let mut tweight7: i32 = 0;
              let mut tweight8: i32 = 0;
              let mut weight1: i32 = simpleListArray2[index13].FindWeight(index15);
              let mut tweight9: i32 = 0;
              if (weight1 > 0)
              {
                let mut weight2: i32 = simpleListArray6[index13].FindWeight(index15);
                let mut weight3: i32 = simpleListArray4[index13].FindWeight(index15);
                let mut weight4: i32 = simpleListArray5[index13].FindWeight(index15);
                let mut weight5: i32 = simpleList5.FindWeight(index15);
                let mut num15: i32 = weight1 - weight4;
                if (num15 < 0)
                  num15 = 0;
                let mut tweight10: i32 =  Math.Round(Math.Floor( (weight3 * weight5) / 100.0));
                tweight9 = tweight10;
                let mut num16: i32 = num15 + tweight10;
                if (num16 >= weight1)
                {
                  tweight7 = 100;
                  tweight8 = 100;
                }
                else if (num16 > weight2 & num16 > 0)
                {
                  tweight7 =  Math.Round(Math.Floor( (100 * (num16 - weight2)) /  weight1));
                  tweight8 = 100;
                }
                else if (weight1 > 0)
                {
                  tweight7 = 0;
                  tweight8 =  Math.Round(Math.Floor( (100 * num16) /  weight2));
                }
                self.PIL.Add(index15, tweight10, 2, num14, weight4, 1, CheckExistence: false);
                if (weight4 > 0)
                {
                  str4: String = "" + weight4.ToString() + " of " + self.itemName[index15] + " is missing in Zone.\r\n";
                  str5: String;
                  if (weight3 < weight4)
                    str5 = str4 + "Only " + weight3.ToString() + " of " + self.itemName[index15] + " is requested at SHQ due to probable lack of Logistical Points.\r\n";
                  else
                    str5 = str4 + weight3.ToString() + " of " + self.itemName[index15] + " is requested at SHQ.\r\n";
                  tid2: String;
                  if (tweight10 < weight3)
                    tid2 = str5 + "Only " + tweight10.ToString() + " " + self.itemName[index15] + " will be delivered Zone due to probable lack of SHQ inventory.";
                  else
                    tid2 = str5 + tweight10.ToString() + " " + self.itemName[index15] + " will be delivered to Zone.";
                  self.PILdesc.Add(tid2, 1, 2, num14, index15, 1, CheckExistence: false);
                }
              }
              else
              {
                tweight7 = 100;
                tweight8 = 100;
              }
              simpleList1.RemoveWeight(index15, tweight9);
              let mut tweight11: i32 = simpleListArray1[index13].FindWeight(index15) + tweight9;
              simpleListArray1[index13].AddWeight(index15, tweight9);
              self.PIL.Add(index15, tweight11, 3, num14, tdata4: 1, CheckExistence: false);
              if (tweight11 > 0)
                self.PILdesc.Add("After any deliveries from SHQ the Zone will have " + tweight11.ToString() + " " + self.itemName[index15] + " in reserve.", 1, 3, num14, index15, 1, CheckExistence: false);
              simpleList6.Add(index15, tweight7);
              simpleList7.Add(index15, tweight8);
              index15 += 1;
            }
            while (index15 <= 30);
            self.game.EventRelatedObj.ExecMakeAssetPresentation("SE_Data", -1, -1, num14, "", prodAdjustmentLists: true, itemsProdModList: ( simpleList6), itemsUpkeepModList: ( simpleList7), logActual: true);
            let mut length5: i32 = self.game.Data.StringListObj[stringListById2].Length;
            for (let mut index16: i32 = 0; index16 <= length5; index16 += 1)
            {
              let mut index17: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index16, 0]));
              if (index17 > 0)
              {
                let mut num17: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index16, 2]));
                let mut tweight12: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index16, 3]));
                let mut tweight13: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index16, 4]));
                let mut num18: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].Data[index16, 8]));
                num19: i32;
                if (tweight12 > 0)
                {
                  simpleListArray7[index13].AddWeight(index17, tweight12);
                  let mut weight6: i32 = simpleListArray2[index13].FindWeight(index17);
                  if (tweight12 > 0 | weight6 > 0)
                  {
                    self.slotDetail = self.slotPreviewAssetLog;
                    str1 = "";
                    str2 = "";
                    Left1: String = "";
                    Left2: String = "";
                    str3 = self.itemName[index17] + " consumed in Zone";
                    str6: String = weight6.ToString() + " could optimally be consumed.\r\n" + tweight12.ToString() + " was actually available and consumed by Zone.";
                    if (weight6 > 0)
                    {
                      Left1 = "Assets that consumed:\r\n";
                      num19 = 0;
                      num12 = 0;
                      if (self.slotDetail > -1)
                      {
                        let mut length6: i32 = self.game.Data.StringListObj[self.slotDetail].Length;
                        for (let mut index18: i32 = 0; index18 <= length6; index18 += 1)
                        {
                          let mut tid3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index18, 0]));
                          if (tid3 < 1000000)
                            tweight1 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[tid3], 0]));
                          let mut num20: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index18, 1]));
                          let mut num21: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index18, 3]));
                          if (tid3 < 1000000)
                          {
                            if (tweight1 == num14 & num20 == 2 & num21 == index17)
                            {
                              let mut num22: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index18, 2]));
                              num13 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index18, 4]));
                              let mut idValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[tid3], 1]));
                              str7: String = self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue, 1);
                              let mut nr: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue, 2)));
                              let mut num23: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[tid3], 11]));
                              if (num22 == 2 | num22 == 4 | num22 == 6)
                              {
                                self.listItemAsset.Add(tid3, 1, 4, index17, CheckExistence: false);
                                if (nr > 0)
                                  str7 = str7 + " " + self.game.HandyFunctionsObj.GetRomanNumerical(nr);
                                Left1 = Left1 + num23.ToString() + "% prod, " + str7 + " consumed " + num13.ToString() + " " + self.itemName[index17] + "\r\n";
                              }
                            }
                          }
                          else if (tid3 >= 1000000)
                          {
                            let mut tid4: i32 = tid3;
                            tweight1 = num14;
                            let mut num24: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index18, 1]));
                            let mut num25: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index18, 3]));
                            if (tweight1 == num14 & num24 == 2 & num25 == index17)
                            {
                              data: String = self.game.Data.StringListObj[stringListById1].GetData(0, tid4 - 1000000, 1);
                              let mut num26: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index18, 2]));
                              num13 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index18, 4]));
                              let mut num27: i32 = self.game.Data.StringListObj[self.slotDetail].Width < 5 ? num14 :  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index18, 5]));
                              if (num26 == 14 & num27 == num14)
                              {
                                self.listItemAsset.Add(tid4, 1, 4, index17, CheckExistence: false);
                                if (Operators.CompareString(Left2, "", false) == 0)
                                  Left2 = "Hex Perks that contributed:\r\n";
                                if (index17 == 9 | index17 == 12)
                                  num13 *= 100;
                                Left2 = Left2 + data + " produced " + num13.ToString() + " " + self.itemName[index17] + "\r\n";
                              }
                            }
                          }
                        }
                      }
                    }
                    if (Left2.Length > 0)
                      Left1 += Left2;
                    if (Operators.CompareString(Left1, "", false) == 0)
                      Left1 = "No assets contributed to this consumption";
                    self.PILdesc.Add(str6 + "\r\n" + Left1, 1, 4, num14, index17, 1, CheckExistence: false);
                  }
                  let mut nr1: i32 = self.PIL.FindNr(index17, 4, num14, tdata4: 1);
                  if (nr1 == -1)
                  {
                    self.PIL.Add(index17, tweight12, 4, num14, weight6, 1, CheckExistence: false);
                  }
                  else
                  {
                    int[] weight7 = self.PIL.Weight;
                    int[] numArray3 = weight7;
                    let mut index19: i32 = nr1;
                    let mut index20: i32 = index19;
                    let mut num28: i32 = weight7[index19] + tweight12;
                    numArray3[index20] = num28;
                  }
                }
                if (tweight13 > 0)
                {
                  simpleListArray8[index13].AddWeight(index17, tweight13);
                  let mut weight8: i32 = simpleListArray3[index13].FindWeight(index17);
                  Left3: String = "";
                  if ((weight8 > 0 | tweight13 > 0) & self.slotPreviewAssetLog > -1)
                  {
                    self.slotDetail = self.slotPreviewAssetLog;
                    str1 = "";
                    str2 = "";
                    Left4: String = "";
                    str3 = self.itemName[index17] + " produced in Zone";
                    str8: String = weight8.ToString() + " could optimally be produced.\r\n" + tweight13.ToString() + " was actually produced by Zone.";
                    if (weight8 > 0)
                    {
                      num19 = 0;
                      num12 = 0;
                      if (self.slotDetail > -1)
                      {
                        let mut length7: i32 = self.game.Data.StringListObj[self.slotDetail].Length;
                        for (let mut index21: i32 = 0; index21 <= length7; index21 += 1)
                        {
                          let mut tid5: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index21, 0]));
                          if (tid5 < 1000000)
                          {
                            tweight1 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[tid5], 0]));
                            let mut num29: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index21, 1]));
                            let mut num30: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index21, 3]));
                            if (tweight1 == num14 & num29 == 2 & num30 == index17)
                            {
                              let mut num31: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index21, 2]));
                              num13 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index21, 4]));
                              let mut idValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[tid5], 1]));
                              str9: String = self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue, 1);
                              let mut nr: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue, 2)));
                              let mut num32: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[tid5], 11]));
                              if (num31 == 14)
                              {
                                self.listItemAsset.Add(tid5, 1, 5, index17, CheckExistence: false);
                                if (Operators.CompareString(Left4, "", false) == 0)
                                  Left4 = "Assets that contributed:\r\n";
                                if (nr > 0)
                                  str9 = str9 + " " + self.game.HandyFunctionsObj.GetRomanNumerical(nr);
                                Left4 = Left4 + num32.ToString() + "% prod, " + str9 + " produced " + num13.ToString() + " " + self.itemName[index17] + "\r\n";
                              }
                            }
                          }
                          else if (tid5 >= 1000000)
                          {
                            tweight1 = num14;
                            let mut num33: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index21, 1]));
                            let mut num34: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index21, 3]));
                            if (tweight1 == num14 & num33 == 2 & num34 == index17)
                            {
                              data: String = self.game.Data.StringListObj[stringListById1].GetData(0, tid5 - 1000000, 1);
                              let mut num35: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index21, 2]));
                              num13 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index21, 4]));
                              let mut num36: i32 = self.game.Data.StringListObj[self.slotDetail].Width < 5 ? num14 :  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index21, 5]));
                              if (num35 == 14 & num36 == num14)
                              {
                                self.listItemAsset.Add(tid5, 1, 5, index17, CheckExistence: false);
                                if (Operators.CompareString(Left3, "", false) == 0)
                                  Left3 = "Hex Perks that contributed:\r\n";
                                if (index17 == 9 | index17 == 12)
                                  num13 *= 100;
                                Left3 = Left3 + data + " produced " + num13.ToString() + " " + self.itemName[index17] + "\r\n";
                              }
                            }
                          }
                        }
                      }
                    }
                    if (Left3.Length > 0)
                      Left4 += Left3;
                    if (Operators.CompareString(Left4, "", false) == 0)
                      Left4 = "No assets contributed to this production";
                    self.PILdesc.Add(str8 + "\r\n" + Left4, 1, 5, num14, index17, 1, CheckExistence: false);
                  }
                  let mut num37: i32 = self.PIL.FindNr(index17, 5, num14, tdata4: 1);
                  if (index17 == 13)
                    num37 = num37;
                  if (num37 == -1)
                  {
                    self.PIL.Add(index17, tweight13, 5, num14, weight8, 1, CheckExistence: false);
                  }
                  else
                  {
                    int[] weight9 = self.PIL.Weight;
                    int[] numArray4 = weight9;
                    let mut index22: i32 = num37;
                    let mut index23: i32 = index22;
                    let mut num38: i32 = weight9[index22] + tweight13;
                    numArray4[index23] = num38;
                  }
                }
              }
            }
            let mut index24: i32 = 0;
            do
            {
              let mut tweight14: i32 = simpleListArray1[index13].FindWeight(index24) + simpleListArray8[index13].FindWeight(index24) - simpleListArray7[index13].FindWeight(index24);
              if (tweight14 < 0)
                tweight14 = 0;
              if (tweight14 > 0)
              {
                self.PIL.Add(index24, tweight14, 6, num14, tdata4: 1, CheckExistence: false);
                self.PILdesc.Add("After consumption and production of Items the Zone will have " + tweight14.ToString() + " " + self.itemName[index24] + " in reserve.", 1, 6, num14, index24, 1, CheckExistence: false);
              }
              index24 += 1;
            }
            while (index24 <= 30);
            simpleListArray1[index13].AddWeight( simpleListArray8[index13]);
            simpleListArray1[index13].RemoveWeight( simpleListArray7[index13]);
            simpleListArray1[index13].removeWeight0orLower();
            if (index14 > -1)
            {
              SimpleList simpleList8 = self.game.Data.LocObj[index14].items.list.Clone();
              self.game.Data.LocObj[index14].items.list = simpleListArray1[index13].Clone();
              self.game.EventRelatedObj.ExecMakeListForLocationReturns("SE_Data", num14, 0, 0, "");
              self.game.Data.LocObj[index14].items.list = simpleList8;
              simpleListArray9[index13].AddWeight( self.game.Data.LocObj[index14].tempRequestItems);
              simpleListArray10[index13] = simpleListArray9[index13].Clone();
              let mut num39: i32 = 0;
              let mut counter8: i32 = simpleListArray9[index13].Counter;
              for (let mut index25: i32 = 0; index25 <= counter8; index25 += 1)
              {
                let mut num40: i32 = self.itemWeight[simpleListArray9[index13].Id[index25]] * simpleListArray9[index13].Weight[index25];
                if (x1 == x3 & y1 == y3)
                  num40 = 0;
                num39 += num40;
              }
              if (num39 > numArray1[index13] | numArray1[index13] == 0)
              {
                let mut counter9: i32 = simpleListArray9[index13].Counter;
                for (let mut index26: i32 = 0; index26 <= counter9; index26 += 1)
                {
                  if (self.itemWeight[simpleListArray9[index13].Id[index26]] > 0 | numArray1[index13] == 0 && !(x1 == x3 & y1 == y3))
                  {
                    let mut num41: i32 =  Math.Round(Math.Floor( (simpleListArray9[index13].Weight[index26] * numArray1[index13]) /  num39));
                    simpleListArray9[index13].Weight[index26] = num41;
                  }
                }
              }
              let mut counter10: i32 = simpleListArray9[index13].Counter;
              for (let mut index27: i32 = 0; index27 <= counter10; index27 += 1)
              {
                let mut index28: i32 = simpleListArray9[index13].Id[index27];
                let mut tweight15: i32 = simpleListArray9[index13].Weight[index27];
                let mut weight: i32 = simpleListArray10[index13].FindWeight(index28);
                self.PIL.Add(index28, tweight15, 7, num14, weight, 1, CheckExistence: false);
                if (weight > 0)
                {
                  str10: String = "Ideally the Zone would like to send a local surplus of " + weight.ToString() + " " + self.itemName[index28] + " to its SHQ.";
                  tid6: String;
                  if (tweight15 >= weight)
                    tid6 = str10 + "\r\nThis is what will probably happen.";
                  else
                    tid6 = str10 + "\r\nDue to probable logistical problems only " + tweight15.ToString() + " " + self.itemName[index28] + " will be sent to its SHQ.";
                  self.PILdesc.Add(tid6, 1, 7, num14, index28, 1, CheckExistence: false);
                }
                simpleList1.AddWeight(index28, tweight15);
              }
            }
          }
        }
      }
      simpleList1.removeWeight0orLower();
      let mut counter11: i32 = self.listZone.Counter;
      for (let mut index29: i32 = 0; index29 <= counter11; index29 += 1)
      {
        if (self.listZone.Data1[index29] == self.game.EditObj.se1_assetSHQ)
        {
          let mut tdata2: i32 = self.listZone.Id[index29];
          let mut counter12: i32 = simpleList1.Counter;
          for (let mut index30: i32 = 0; index30 <= counter12; index30 += 1)
            self.PIL.Add(simpleList1.Id[index30], simpleList1.Weight[index30], 8, tdata2, tdata4: 1, CheckExistence: false);
        }
      }
      self.IL = SimpleList::new();
      self.ILdesc = SimpleStringList::new();
      let mut counter13: i32 = self.listZone.Counter;
      for (let mut index31: i32 = 0; index31 <= counter13; index31 += 1)
      {
        let mut num42: i32 = self.listZone.Id[index31];
        Conversions.ToInteger(self.game.Data.StringListObj[self.slotZones].GetData(0, num42, 1));
        Conversions.ToInteger(self.game.Data.StringListObj[self.slotZones].GetData(0, num42, 2));
        let mut id: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZones].GetData(0, num42, 6)));
        self.game.Data.StringListObj[self.slotZones].GetData(0, num42, 7);
        let mut index32: i32 = self.game.EventRelatedObj.CheckRegimeSlot( Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZones].GetData(0, num42, 8))), 0, 0, 0);
        locationById: i32;
        if (id > 0)
          locationById = self.game.HandyFunctionsObj.GetLocationByID(id);
        let mut num43: i32 = -1;
        if (locationById > -1)
          num43 = self.game.Data.LocObj[locationById].HQ;
        let mut num44: i32 = -1;
        if (num42 > 0 & index32 > -1)
          num44 = self.game.EventRelatedObj.Helper_GetCharacterId(self.game.Data.RegimeObj[index32].id, 10, num42, 0);
        if (locationById > -1)
        {
          SimpleList simpleList9 = SimpleList::new();
          SimpleList simpleList10 = SimpleList::new();
          SimpleStringList simpleStringList = SimpleStringList::new();
          let mut tdata1_1: i32 = 0;
          let mut tdata2_1: i32 = 0;
          simpleList9.Add(7, 0, tdata1_1, tdata2_1);
          simpleList10.Add(19, 0, CheckExistence: false);
          simpleStringList.Add("Food", 1);
          let mut tdata2_2: i32 = tdata2_1 + 1;
          simpleList9.Add(5, 0, tdata1_1, tdata2_2);
          simpleList10.Add(20, 0, CheckExistence: false);
          simpleStringList.Add("Water", 1);
          let mut tdata2_3: i32 = tdata2_2 + 1;
          simpleList9.Add(1, 0, tdata1_1, tdata2_3);
          simpleList10.Add(18, 0, CheckExistence: false);
          simpleStringList.Add("Fuel", 1);
          let mut tdata2_4: i32 = tdata2_3 + 1;
          simpleList9.Add(10, 0, tdata1_1, tdata2_4);
          simpleList10.Add(17, 0, CheckExistence: false);
          simpleStringList.Add("Ammo", 1);
          let mut tdata2_5: i32 = tdata2_4 + 1;
          simpleList9.Add(15, 0, tdata1_1, tdata2_5);
          simpleList10.Add(16, 0, CheckExistence: false);
          simpleStringList.Add("Energy", 1);
          let mut tdata1_2: i32 = tdata1_1 + 1;
          let mut tdata2_6: i32 = 0;
          simpleList9.Add(2, 0, tdata1_2, tdata2_6);
          simpleList10.Add(2, 0, CheckExistence: false);
          simpleStringList.Add("Metals", 1);
          let mut tdata2_7: i32 = tdata2_6 + 1;
          simpleList9.Add(3, 0, tdata1_2, tdata2_7);
          simpleList10.Add(3, 0, CheckExistence: false);
          simpleStringList.Add("Rare Metals", 1);
          let mut tdata2_8: i32 = tdata2_7 + 1;
          simpleList9.Add(13, 0, tdata1_2, tdata2_8);
          simpleList10.Add(13, 0, CheckExistence: false);
          simpleStringList.Add("Machines", 1);
          let mut tdata2_9: i32 = tdata2_8 + 1;
          simpleList9.Add(14, 0, tdata1_2, tdata2_9);
          simpleList10.Add(14, 0, CheckExistence: false);
          simpleStringList.Add("Hi-Tech Parts", 1);
          let mut tdata2_10: i32 = tdata2_9 + 1;
          simpleList9.Add(8, 0, tdata1_2, tdata2_10);
          simpleList10.Add(22, 0, CheckExistence: false);
          simpleStringList.Add("Industrial Points", 1);
          let mut tdata1_3: i32 = tdata1_2 + 1;
          let mut tdata2_11: i32 = 0;
          simpleList9.Add(9, 0, tdata1_3, tdata2_11);
          simpleList10.Add(9, 0, CheckExistence: false);
          simpleStringList.Add("Recruits", 1);
          let mut tdata2_12: i32 = tdata2_11 + 1;
          simpleList9.Add(12, 0, tdata1_3, tdata2_12);
          simpleList10.Add(12, 0, CheckExistence: false);
          simpleStringList.Add("Colonists", 1);
          let mut tdata2_13: i32 = tdata2_12 + 1;
          simpleList9.Add(4, 0, tdata1_3, tdata2_13);
          simpleList10.Add(4, 0, CheckExistence: false);
          simpleStringList.Add("Radioactives", 1);
          let mut counter14: i32 = simpleList9.Counter;
          for (let mut index33: i32 = 0; index33 <= counter14; index33 += 1)
          {
            str11: String = simpleStringList.Id[index33];
            let mut num45: i32 = simpleList9.Id[index33];
            let mut num46: i32 = simpleList10.Id[index33];
            if (num45 == num46)
              ;
            let mut tweight16: i32 = 0;
            let mut num47: i32 = 0;
            let mut num48: i32 = 0;
            let mut num49: i32 = 0;
            let mut num50: i32 = 0;
            let mut tdata3_1: i32 = 0;
            let mut num51: i32 = 0;
            let mut tdata3_2: i32 = 0;
            let mut tdata3_3: i32 = 0;
            let mut num52: i32 = 0;
            let mut num53: i32 = 0;
            let mut num54: i32 = 0;
            let mut num55: i32 = 0;
            let mut num56: i32 = 0;
            let mut logCounter: i32 = self.game.Data.LocObj[locationById].LogCounter;
            for (let mut index34: i32 = 0; index34 <= logCounter; index34 += 1)
            {
              if (self.game.Data.LocObj[locationById].LogData1[index34] == num45)
              {
                if (self.game.Data.LocObj[locationById].LogType[index34] == 101)
                  tweight16 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 104)
                  num47 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 103)
                  num49 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 102)
                  num50 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 121)
                  num48 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 201)
                  tdata3_1 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 204)
                  num51 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 203)
                  tdata3_2 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 202)
                  tdata3_3 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 401)
                  num53 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 402)
                {
                  num52 += self.game.Data.LocObj[locationById].LogData3[index34];
                  num49 += self.game.Data.LocObj[locationById].LogData3[index34];
                }
                if (self.game.Data.LocObj[locationById].LogType[index34] == 403)
                  num54 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 404)
                  num55 += self.game.Data.LocObj[locationById].LogData3[index34];
                if (self.game.Data.LocObj[locationById].LogType[index34] == 405)
                  num56 += self.game.Data.LocObj[locationById].LogData3[index34];
              }
            }
            if (num45 == 9 | num45 == 12)
            {
              tweight16 *= 100;
              num47 *= 100;
              num48 *= 100;
              num49 *= 100;
              num50 *= 100;
              tdata3_1 *= 100;
              num51 *= 100;
              tdata3_2 *= 100;
              tdata3_3 *= 100;
              num54 *= 100;
              num56 *= 100;
            }
            tweight17: i32;
            double num57;
            eventPicSlotFor: i32;
            if (tweight16 > 0 | tdata3_1 > 0)
            {
              tweight17 = tweight16;
              tid7: String = tweight17.ToString();
              if (tweight17 > 999)
              {
                num57 = Math.Round( tweight17 / 1000.0, 1);
                tid7 = num57.ToString() + "k";
              }
              eventPicSlotFor = self.game.EventRelatedObj.GetEventPicSlotFor(num45, "", "");
              str3 = simpleStringList.Id[index33] + " sent from SHQ to Zone";
              str1 = tdata3_1.ToString() + " requested by Zone.\r\n" + tweight16.ToString() + " sent by SHQ.";
              if (tweight16 > 0)
                tid7 = "+" + tid7;
              self.IL.Add(num45, tweight16, 2, num42, tdata3_1, CheckExistence: false);
              self.ILdesc.Add(tid7, 1, 2, num42, num45, CheckExistence: false);
            }
            Left5: String = "";
            if (num47 > 0 | num51 > 0 | num48 > 0)
            {
              tweight17 = num47 + num48;
              str12: String = tweight17.ToString();
              if (tweight17 > 999)
              {
                num57 = Math.Round( tweight17 / 1000.0, 1);
                str12 = num57.ToString() + "k";
              }
              if (num47 > 0)
                str12 = "+" + str12;
              eventPicSlotFor = self.game.EventRelatedObj.GetEventPicSlotFor(num45, "", "");
              str3 = simpleStringList.Id[index33] + " produced in Zone";
              str13: String = num51.ToString() + " could optimally be produced.\r\n" + num47.ToString() + " was actually produced by Zone.";
              if (num48 > 0)
                str13 = str13 + "\r\n" + num48.ToString() + " was delivered by Zone Militia.";
              str2 = str12;
              Left6: String = "";
              if (num51 > 0)
              {
                tweight17 = 0;
                num12 = 0;
                if (self.slotDetail > -1)
                {
                  let mut length8: i32 = self.game.Data.StringListObj[self.slotDetail].Length;
                  for (let mut index35: i32 = 0; index35 <= length8; index35 += 1)
                  {
                    let mut index36: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index35, 0]));
                    if (index36 < 1000000)
                    {
                      let mut num58: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[index36], 0]));
                      let mut num59: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index35, 1]));
                      let mut num60: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index35, 3]));
                      if (num58 == num42 & num59 == 2 & num60 == num45)
                      {
                        let mut num61: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index35, 2]));
                        num13 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index35, 4]));
                        let mut idValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[index36], 1]));
                        str14: String = self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue, 1);
                        let mut nr: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue, 2)));
                        let mut num62: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[index36], 11]));
                        if (num61 == 14)
                        {
                          if (Operators.CompareString(Left6, "", false) == 0)
                            Left6 = "Assets that contributed:\r\n";
                          if (nr > 0)
                            str14 = str14 + " " + self.game.HandyFunctionsObj.GetRomanNumerical(nr);
                          Left6 = Left6 + num62.ToString() + "% prod, " + str14 + " produced " + num13.ToString() + " " + self.game.Data.StringListObj[self.slotItemType].GetData(0, num45, 1) + "\r\n";
                        }
                      }
                    }
                    else if (index36 >= 1000000)
                    {
                      let mut num63: i32 = num42;
                      let mut num64: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index35, 1]));
                      let mut num65: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index35, 3]));
                      if (num63 == num42 & num64 == 2 & num65 == num45)
                      {
                        data: String = self.game.Data.StringListObj[stringListById1].GetData(0, index36 - 1000000, 1);
                        let mut num66: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index35, 2]));
                        num13 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index35, 4]));
                        let mut num67: i32 = self.game.Data.StringListObj[self.slotDetail].Width < 5 ? num42 :  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index35, 5]));
                        if (num66 == 14 & num67 == num42)
                        {
                          if (Operators.CompareString(Left5, "", false) == 0)
                            Left5 = "Hex Perks that contributed:\r\n";
                          if (num45 == 9 | num45 == 12)
                            num13 *= 100;
                          Left5 = Left5 + data + " produced " + num13.ToString() + " " + self.game.Data.StringListObj[self.slotItemType].GetData(0, num45, 1) + "\r\n";
                        }
                      }
                    }
                  }
                }
              }
              if (Left5.Length > 0)
                Left6 += Left5;
              if (num54 > 0)
                Left6 = Left6 + "Recruitment in zone contributed " + num54.ToString() + " " + self.game.Data.StringListObj[self.slotItemType].GetData(0, num45, 1) + "\r\n";
              if (num55 > 0)
                Left6 = Left6 + "Service Tax contributed " + num55.ToString() + " " + self.game.Data.StringListObj[self.slotItemType].GetData(0, num45, 1) + "\r\n";
              if (num56 > 0)
                Left6 = Left6 + "Free Production/Collection contributed " + num56.ToString() + " " + self.game.Data.StringListObj[self.slotItemType].GetData(0, num45, 1) + "\r\n";
              if (Operators.CompareString(Left6, "", false) == 0)
                Left6 = "No assets contributed to this production";
              tid8: String = str13 + "\r\n" + Left6;
              tweight17 = num47 + num48;
              self.IL.Add(num45, tweight17, 5, num42, num48 + num51, CheckExistence: false);
              self.ILdesc.Add(tid8, 1, 5, num42, num45, CheckExistence: false);
            }
            if (num49 > tdata3_2)
              tdata3_2 = num49;
            str15: String;
            if (num49 > 0 | tdata3_2 > 0)
            {
              tweight17 = num49;
              str16: String = tweight17.ToString();
              if (tweight17 > 999)
              {
                num57 = Math.Round( tweight17 / 1000.0, 1);
                str16 = num57.ToString() + "k";
              }
              if (num49 > 0)
                str16 = "-" + str16;
              eventPicSlotFor = self.game.EventRelatedObj.GetEventPicSlotFor(num45, "", "");
              str3 = simpleStringList.Id[index33] + " consumed in Zone";
              str17: String = tdata3_2.ToString() + " could optimally be consumed.\r\n" + num49.ToString() + " was actually available and consumed by Zone.";
              str2 = str16;
              str15 = "";
              Left7: String = "";
              if (tdata3_2 > 0)
              {
                Left7 = "Assets that consumed:\r\n";
                tweight17 = 0;
                num12 = 0;
                if (self.slotDetail > -1)
                {
                  let mut length9: i32 = self.game.Data.StringListObj[self.slotDetail].Length;
                  for (let mut index37: i32 = 0; index37 <= length9; index37 += 1)
                  {
                    let mut index38: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index37, 0]));
                    if (index38 < 1000000)
                    {
                      let mut num68: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[index38], 0]));
                      let mut num69: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index37, 1]));
                      let mut num70: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index37, 3]));
                      if (num68 == num42 & num69 == 2 & num70 == num45)
                      {
                        let mut num71: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index37, 2]));
                        num13 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotDetail].Data[index37, 4]));
                        let mut idValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[index38], 1]));
                        str18: String = self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue, 1);
                        let mut nr: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue, 2)));
                        let mut num72: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[numArray2[index38], 11]));
                        if (num71 == 2 | num71 == 4 | num71 == 6)
                        {
                          if (nr > 0)
                            str18 = str18 + " " + self.game.HandyFunctionsObj.GetRomanNumerical(nr);
                          Left7 = Left7 + num72.ToString() + "% prod, " + str18 + " consumed " + num13.ToString() + " " + self.game.Data.StringListObj[self.slotItemType].GetData(0, num45, 1) + "\r\n";
                        }
                      }
                    }
                  }
                }
              }
              if (num53 > 0)
                Left7 = Left7 + "Workers consumed " + num53.ToString() + " " + self.game.Data.StringListObj[self.slotItemType].GetData(0, num45, 1) + "\r\n";
              if (num52 > 0)
                Left7 = Left7 + "Population was given " + num52.ToString() + " " + self.game.Data.StringListObj[self.slotItemType].GetData(0, num45, 1) + "\r\n";
              if (Operators.CompareString(Left7, "", false) == 0)
                Left7 = "No assets contributed to this consumption";
              tid9: String = str17 + "\r\n" + Left7;
              tweight17 = num49;
              self.IL.Add(num45, tweight17, 4, num42, tdata3_2, CheckExistence: false);
              self.ILdesc.Add(tid9, 1, 4, num42, num45, CheckExistence: false);
            }
            if (num50 > 0 | tdata3_3 > 0)
            {
              tweight17 = num50;
              str15 = tweight17.ToString();
              if (tweight17 > 999)
              {
                num57 = Math.Round( tweight17 / 1000.0, 1);
                str15 = num57.ToString() + "k";
              }
              str3 = simpleStringList.Id[index33] + " delivered from Zone to SHQ";
              tid10: String = tdata3_3.ToString() + " could optimally be delivered to SHQ.\r\n" + num50.ToString() + " was actually delivered by Zone.";
              self.IL.Add(num45, tweight17, 7, num42, tdata3_3, CheckExistence: false);
              self.ILdesc.Add(tid10, 1, 7, num42, num45, CheckExistence: false);
            }
            let mut num73: i32 = 0;
            let mut num74: i32 = 0;
            let mut tdata3_4: i32 = 0;
            if (Information.IsNothing( self.game.Data.LocObj[locationById].items))
              self.game.Data.LocObj[locationById].items = ItemList::new();
            if (Information.IsNothing( self.game.Data.LocObj[locationById].items.list))
              self.game.Data.LocObj[locationById].items.list = SimpleList::new();
            let mut weight: i32 = self.game.Data.LocObj[locationById].items.list.FindWeight(num45);
            let mut num75: i32 = 0;
            let mut num76: i32 = 0;
            let mut num77: i32 = 0;
            if (weight > 0)
            {
              tweight17 = weight;
              if (num45 == 9 | num45 == 12)
                tweight17 *= 100;
              str15 = tweight17.ToString();
              if (tweight17 > 999)
              {
                num57 = Math.Round( tweight17 / 1000.0, 1);
                str15 = num57.ToString() + "k";
              }
              str3 = simpleStringList.Id[index33] + " Zone Stocks";
              tid11: String = "This Zone has " + weight.ToString() + " " + simpleStringList.Id[index33] + " in reserve Stock.";
              if (num75 > 0)
                tid11 = tid11 + "\r\nLost " + (num75 - num76).ToString() + " items due to exceeding maximum stockage in Zone.";
              if (num76 > 0)
                tid11 = tid11 + "\r\nSold " + num76.ToString() + " items for " + num77.ToString() + " Credits due to exceeding maximum stockage in Zone.";
              if (num73 > 0)
                tid11 = tid11 + "\r\nZone provided " + num73.ToString() + " " + simpleStringList.Id[index33] + " Stockage.";
              if (num74 > 0)
                tid11 = tid11 + "\r\nOf these the Zone provided " + num74.ToString() + " " + simpleStringList.Id[index33] + " Stockage to its SHQ.";
              color: Color = self.game.seColWhite;
              if (num75 > 0)
                color = self.game.seColYellow;
              self.IL.Add(num45, tweight17, 8, num42, tdata3_4, CheckExistence: false);
              self.ILdesc.Add(tid11, 1, 8, num42, num45, CheckExistence: false);
            }
          }
        }
      }
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            self.SubPartList[index].DescriptInfo(x - self.SubPartX[index], y - self.SubPartY[index]);
            if (Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
            {
              self.game.EditObj.TipButton = true;
              self.game.EditObj.TipTitle = "";
              self.game.EditObj.TipText = self.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 0)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          break;
        }
      }
    }

    pub fn dostuff(bool crmAlreadySet = false)
    {
      SizeF sizeF = SizeF::new();
      let mut id1: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      libName1: String = "SE_Data";
      self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName1, 225, 0, 0));
      self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName1, 228, 0, 0));
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName1, 169, 0, 0));
      self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName1, 168, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName1, 143, 0, 0));
      let mut stringListById3: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 306, 0, 0));
      self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName1, 190, 0, 0));
      let mut turn: i32 = self.game.Data.Turn;
      let mut idValue1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, id1, 2)));
      let mut num1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].GetData(0, idValue1, 1)));
      self.game.Data.StringListObj[stringListById1].SetData(0, "REGIMEID", 1, id1);
      self.game.Data.StringListObj[stringListById1].SetData(0, "ROUND", 1, self.game.Data.Round);
      self.ClearMouse();
      self.NewBackGroundAndClearAll(self.useWidth, self.useHeight, -1);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawRepeatingBackground(g, DrawMod.TGame.BACKGROUND3MARC, 0, 0, self.useWidth, self.useHeight);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      if (self.listId > 0)
      {
        self.RemoveSubPart(self.listId);
        self.listId = 0;
      }
      if (self.list2Id > 0)
      {
        self.RemoveSubPart(self.list2Id);
        self.list2Id = 0;
      }
      if (self.okId > 0)
      {
        self.RemoveSubPart(self.okId);
        self.okId = 0;
      }
      if (self.RightId > 0)
      {
        self.RemoveSubPart(self.RightId);
        self.RightId = 0;
      }
      if (self.StartTurnId > 0)
      {
        self.RemoveSubPart(self.StartTurnId);
        self.StartTurnId = 0;
      }
      if (self.PreviewId > 0)
      {
        self.RemoveSubPart(self.PreviewId);
        self.PreviewId = 0;
      }
      if (self.RightId2 > 0)
      {
        self.RemoveSubPart(self.RightId2);
        self.RightId2 = 0;
      }
      if (self.StartTurnId2 > 0)
      {
        self.RemoveSubPart(self.StartTurnId2);
        self.StartTurnId2 = 0;
      }
      if (self.PreviewId2 > 0)
      {
        self.RemoveSubPart(self.PreviewId2);
        self.PreviewId2 = 0;
      }
      let mut assetButtonCounter1: i32 = self.assetButtonCounter;
      for (let mut index: i32 = 0; index <= assetButtonCounter1; index += 1)
      {
        self.RemoveSubPart(self.assetButton[index]);
        self.assetButton[index] = 0;
        self.assetButtonData[index] = 0;
      }
      self.assetButtonCounter = -1;
      let mut y1: i32 = 80;
      Rectangle rectangle1 = Rectangle::new(0, y1, self.useWidth, 50);
      Rectangle rectangle2 = Rectangle::new(0, self.useHeight - 50, self.useWidth, 50);
      Rectangle rectangle3 = Rectangle::new(0, y1 + rectangle1.Height, 200, self.useHeight - (rectangle1.Height + rectangle2.Height + y1));
      Rectangle rectangle4 = Rectangle::new(self.useWidth - 500, y1 + rectangle1.Height, 500, self.useHeight - (rectangle1.Height + rectangle2.Height + 150 + y1));
      Rectangle rectangle5 = Rectangle::new(rectangle3.Width, self.useHeight - (rectangle2.Height + 150), self.useWidth - rectangle3.Width, 150);
      let mut width1: i32 = rectangle3.Width;
      let mut y2: i32 = y1 + rectangle1.Height;
      let mut width2: i32 = self.useWidth - (rectangle3.Width + rectangle4.Width);
      let mut height1: i32 = self.useHeight - (rectangle1.Height + rectangle2.Height + rectangle5.Height);
      if (self.game.EditObj.se1_assetRightPanel < 1)
        width2 = self.useWidth - rectangle3.Width;
      Rectangle rectangle6 = Rectangle::new(width1, y2, width2, height1);
      Rectangle rectangle7 = Rectangle::new(rectangle3.Left, rectangle3.Top, rectangle3.Width, 150);
      Rectangle rectangle8 = Rectangle::new(rectangle3.Left, rectangle7.Bottom + 10, rectangle3.Width, rectangle3.Height - 380);
      Rectangle rectangle9 = Rectangle::new(rectangle3.Left, rectangle8.Bottom + 10, rectangle3.Width, 200);
      DrawMod.DrawBlock( g, rectangle3.X, rectangle3.Y, rectangle3.Width, rectangle3.Height, 0, 0, 0, 120);
      DrawMod.DrawBlock( g, rectangle1.X, rectangle1.Y, rectangle1.Width, rectangle1.Height, 0, 0, 0, 180);
      DrawMod.DrawBlock( g, rectangle2.X, rectangle2.Y, rectangle2.Width, rectangle2.Height, 0, 0, 0, 180);
      DrawMod.DrawBlock( g, rectangle5.X, rectangle5.Y, rectangle5.Width, rectangle5.Height, 0, 0, 0, 60);
      if (self.game.EditObj.se1_assetRightPanel > 0)
        DrawMod.DrawBlock( g, rectangle4.X, rectangle4.Y, rectangle4.Width, rectangle4.Height, 0, 0, 0, 120);
      self.ListObj = ListClass::new();
      let mut left1: i32 = rectangle7.Left;
      let mut top1: i32 = rectangle7.Top;
      let mut width3: i32 = rectangle7.Width;
      let mut tlistsize1: i32 =  Math.Round(Math.Floor( rectangle7.Height / 20.0)) - 1;
      let mut tlistselect1: i32 = -1;
      let mut num2: i32 = -1;
      if (self.anyZoneWithoutSHQ)
        self.ListObj.add("No SHQ", 0);
      let mut counter1: i32 = self.listShq.Counter;
      for (let mut index: i32 = 0; index <= counter1; index += 1)
      {
        num2 += 1;
        if (self.game.EditObj.se1_assetSHQ < 1 & !self.anyZoneWithoutSHQ)
          self.game.EditObj.se1_assetSHQ = self.listShq.Id[index];
        if (self.listShq.Id[index] == self.game.EditObj.se1_assetSHQ)
          tlistselect1 = num2;
        self.ListObj.add(self.game.Data.UnitObj[self.listShq.Id[index]].Name, self.listShq.Id[index]);
      }
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(self.ListObj, tlistsize1, width3, tlistselect1, self.game, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: left1, bby: top1, tMarcStyle: true, overruleFont: ( self.game.MarcFont7), overruleItemSize: 20);
      self.listId = self.AddSubPart( tsubpart1, left1, top1, width3, (tlistsize1 + 1) * 20, 1);
      self.List2Obj = ListClass::new();
      let mut left2: i32 = rectangle8.Left;
      let mut top2: i32 = rectangle8.Top;
      let mut width4: i32 = rectangle8.Width;
      let mut tlistsize2: i32 =  Math.Round(Math.Floor( rectangle8.Height / 20.0)) - 1;
      let mut tlistselect2: i32 = -1;
      let mut num3: i32 = -1;
      let mut counter2: i32 = self.listZone.Counter;
      for (let mut index: i32 = 0; index <= counter2; index += 1)
      {
        if (self.listZone.Data1[index] == self.game.EditObj.se1_assetSHQ)
        {
          num3 += 1;
          if (self.game.EditObj.se1_assetZone < 1)
            self.game.EditObj.se1_assetZone = self.listZone.Id[index];
          else if (self.listZone.FindNr(self.game.EditObj.se1_assetZone) == -1)
            self.game.EditObj.se1_assetZone = self.listZone.Id[index];
          if (self.listZone.Id[index] == self.game.EditObj.se1_assetZone)
            tlistselect2 = num3;
          self.List2Obj.add(self.game.Data.StringListObj[self.slotZones].GetData(0, self.listZone.Id[index], 7), self.listZone.Id[index], Conversions.ToString(self.listZone.Weight[index]));
        }
      }
      let mut tsubpart2: SubPartClass =  new ListSubPartClass(self.List2Obj, tlistsize2, width4, tlistselect2, self.game, tShowPair: true, tValueWidth: 70, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: left2, bby: top2, tMarcStyle: true, overruleFont: ( self.game.MarcFont7), overruleItemSize: 20);
      self.list2Id = self.AddSubPart( tsubpart2, left2, top2, width4, (tlistsize2 + 1) * 20, 1);
      let mut x1: i32 = rectangle1.Left +  Math.Round( rectangle1.Width / 2.0);
      let mut y3: i32 = rectangle1.Top + 10;
      tstring1: String = "Zone: " + (self.game.EditObj.se1_assetZone <= 0 ? "None" : self.game.Data.StringListObj[self.slotZones].GetData(0, self.game.EditObj.se1_assetZone, 7));
      if (self.game.EditObj.se1_assetMode == 2)
        tstring1 = "Preview " + tstring1;
      DrawMod.DrawTextColouredMarcCenter( g, tstring1, self.game.MarcFont2, x1, y3, Color.White);
      let mut num4: i32 = rectangle1.Right - 330;
      let mut num5: i32 = y1 + 2;
      let mut num6: i32 = 100;
      let mut num7: i32 = 46;
      bool tred1 = false;
      let mut overlay1: i32 = 1;
      if (self.game.EditObj.se1_assetMode <= 1)
      {
        tred1 = true;
        overlay1 = 0;
      }
      if (overlay1 == 1)
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Start Turn", num6, "Data shown in right-pane will be based on the start of your turn.",  self.OwnBitmap, num4, num5, tred: tred1, theight: num7, useshadow: true, tMarcStyle: true);
        self.StartTurnId = self.AddSubPart( tsubpart3, num4, num5, num6, num7, overlay1);
      }
      else
      {
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Start Turn", num6, tBackbitmap: ( self.OwnBitmap), bbx: num4, bby: num5, tred: tred1, theight: num7, useshadow: true, tMarcStyle: true);
        self.StartTurnId2 = self.AddSubPart( tsubpart4, num4, num5, num6, num7, overlay1);
      }
      let mut num8: i32 = num4 + 110;
      bool tred2 = false;
      let mut overlay2: i32 = 1;
      if (self.game.EditObj.se1_assetMode == 2)
      {
        tred2 = true;
        overlay2 = 0;
      }
      if (overlay2 == 1)
      {
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Preview", num6, "Data shown in right-pane will be based on a preview calculation for your next turn start.",  self.OwnBitmap, num8, num5, tred: tred2, theight: num7, useshadow: true, tMarcStyle: true);
        self.PreviewId = self.AddSubPart( tsubpart5, num8, num5, num6, num7, overlay2);
      }
      else
      {
        let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Preview", num6, tBackbitmap: ( self.OwnBitmap), bbx: num8, bby: num5, tred: tred2, theight: num7, useshadow: true, tMarcStyle: true);
        self.PreviewId2 = self.AddSubPart( tsubpart6, num8, num5, num6, num7, overlay2);
      }
      let mut num9: i32 = num8 + 110;
      bool tred3 = false;
      let mut overlay3: i32 = 1;
      if (self.game.EditObj.se1_assetRightPanel == 1)
      {
        let mut tsubpart7: SubPartClass =  new TextButtonPartClass("Collapse", num6, "Will hide the right panel and free up space to show more Assets.",  self.OwnBitmap, num9, num5, tred: tred3, theight: num7, useshadow: true, tMarcStyle: true);
        self.RightId = self.AddSubPart( tsubpart7, num9, num5, num6, num7, overlay3);
      }
      else
      {
        let mut tsubpart8: SubPartClass =  new TextButtonPartClass("Expand", num6, "Will show the right panel to allow you to inspect Item inventory reports/previews.",  self.OwnBitmap, num9, num5, tred: tred3, theight: num7, useshadow: true, tMarcStyle: true);
        self.RightId = self.AddSubPart( tsubpart8, num9, num5, num6, num7, overlay3);
      }
      let mut num10: i32 = self.game.EditObj.se1_SelectAssetButton;
      let mut se1AssetZone: i32 = self.game.EditObj.se1_assetZone;
      let mut id2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZones].GetData(0, self.game.EditObj.se1_assetZone, 6)));
      let mut index1: i32 = -1;
      let mut num11: i32 = -1;
      let mut num12: i32 = -1;
      num13: i32;
      if (id2 > 0)
      {
        index1 = self.game.HandyFunctionsObj.GetLocationByID(id2);
        if (index1 > -1)
        {
          num11 = self.game.Data.LocObj[index1].X;
          num12 = self.game.Data.LocObj[index1].Y;
        }
        else
          num13 = 0;
      }
      let mut id3: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      let mut num14: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotRegKey].GetData2(0, id3, 1, "credits", 2)));
      self.orderfeedbackString = "";
      num15: i32;
      if (self.AssetOrderNumber > 0)
      {
        if (self.AssetOrderNumber == 32)
        {
          self.game.Data.StringListObj[self.slotAssets].SetData(9, num10, 5, -1);
          self.game.Data.StringListObj[self.slotAssets].SetData(9, num10, 15, 0);
        }
        if (self.AssetOrderNumber == 31)
        {
          let mut setValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0,  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].GetData(9, num10, 1))), 11)));
          self.game.Data.StringListObj[self.slotAssets].SetData(9, num10, 5, setValue);
          self.game.Data.StringListObj[self.slotAssets].SetData(9, num10, 15, 0);
        }
        if (self.AssetOrderNumber == 33)
        {
          self.game.Data.StringListObj[self.slotAssets].SetData(9, num10, 5, -2);
          self.game.Data.StringListObj[self.slotAssets].SetData(9, num10, 15, 0);
        }
        if (self.AssetOrderNumber >= 2000 & self.AssetOrderNumber <= 2100)
        {
          let mut setValue: i32 = self.AssetOrderNumber - 2000;
          if (setValue == 100)
            setValue = 0;
          self.game.Data.StringListObj[self.slotAssets].SetData(9, num10, 15, setValue);
        }
        if (self.AssetOrderNumber == 21)
        {
          let mut idValue2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].GetData(9, num10, 1)));
          let mut num16: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue2, 25)));
          let mut setValue1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, num16, 11)));
          let mut num17: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotConstruction].GetData3(0, idValue2, 1, 3, 2, "credits", 3)));
          let mut setValue2: i32 = num14 - num17;
          let mut num18: i32 =  Math.Round( num17 / 2.0);
          self.game.Data.StringListObj[self.slotRegKey].SetData2(0, id3, 1, "credits", 2, setValue2);
          let mut setValue3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZoneKeys].GetData2(0, se1AssetZone, 1, "popCredits", 2))) + num18;
          let mut num19: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZoneKeys].GetData2(0, se1AssetZone, 1, "popHapiness", 2)));
          let mut num20: i32 = num19;
          let mut setValue4: i32 =  Math.Round( num19 * 0.8);
          let mut num21: i32 = num20 - setValue4;
          self.game.Data.StringListObj[self.slotZoneKeys].SetData2(0, se1AssetZone, 1, "popCredits", 2, setValue3, true);
          self.game.Data.StringListObj[self.slotZoneKeys].SetData2(0, se1AssetZone, 1, "popHapiness", 2, setValue4, true);
          self.game.Data.StringListObj[self.slotAssets].SetData(9, num10, 1, num16);
          self.game.Data.StringListObj[self.slotAssets].SetData(9, num10, 5, setValue1);
          self.orderfeedbackString = self.orderfeedbackString + "Asset was nationalized. Population happiness dropped with " + num21.ToString() + " points.";
        }
        if (self.AssetOrderNumber == 23)
        {
          let mut idValue3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].GetData(9, num10, 1)));
          num15 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].GetData(9, num10, 7)));
          let mut num22: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue3, 13)));
          let mut num23: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].GetData(9, num10, 3)));
          let mut num24: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].GetData(9, num10, 4)));
          SimpleList simpleList = SimpleList::new();
          if (index1 > -1)
          {
            let mut length: i32 = self.game.Data.StringListObj[self.slotPaid].Length;
            for (let mut index2: i32 = 0; index2 <= length; index2 += 1)
            {
              if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotPaid].Data[index2, 0])) == num10 &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotPaid].Data[index2, 1])) == 1)
              {
                let mut tid: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotPaid].Data[index2, 2]));
                let mut tweight: i32 =  Math.Round(  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotPaid].Data[index2, 3])) / 2.0);
                if (tweight > 0)
                  simpleList.AddWeight(tid, tweight);
              }
            }
          }
          self.orderfeedbackString += "Construction was canceled. ";
          if (simpleList.Counter > -1)
          {
            let mut counter3: i32 = simpleList.Counter;
            for (let mut index3: i32 = 0; index3 <= counter3; index3 += 1)
            {
              if (index3 == 0)
                self.orderfeedbackString += "Recuperated: ";
              if (index3 > 0)
                self.orderfeedbackString += ", ";
              data: String = self.game.Data.StringListObj[self.slotItemType].GetData(0, simpleList.Id[index3], 1);
              self.orderfeedbackString = self.orderfeedbackString + simpleList.Weight[index3].ToString() + "x " + data;
              self.game.Data.LocObj[index1].items.list.AddWeight(simpleList.Id[index3], simpleList.Weight[index3]);
            }
          }
          self.game.Data.StringListObj[self.slotAssets].RemoveRow(self.game.Data.StringListObj[self.slotAssets].FindRow(9, num10));
          self.game.EventRelatedObj.Helper_SetLocationTypeForHex(num23, num24, num23, num24);
        }
        self.ReCalculate();
      }
      self.AssetOrderNumber = 0;
      let mut num25: i32 = 0;
      let mut num26: i32 =  Math.Round(Math.Floor( rectangle6.Width / 160.0)) *  Math.Round(Math.Floor( rectangle6.Height / 210.0));
      num15 = -1;
      self.game.Data.FindEventPic("", 0, "SE_Present");
      self.game.Data.FindEventPic("", 109, "SE_Present");
      let mut num27: i32 = -1;
      self.game.Data.FindEventPic("", 5, "SE_Present");
      let mut num28: i32 = -1;
      int[] numArray1 = new int[10];
      SimpleList simpleList1 = SimpleList::new();
      let mut num29: i32 = 1;
      num30: i32;
      integer1: i32;
      num31: i32;
      num32: i32;
      num33: i32;
      num34: i32;
      do
      {
        let mut length: i32 = self.game.Data.StringListObj[self.slotAssets].Length;
        for (let mut tid1: i32 = 0; tid1 <= length; tid1 += 1)
        {
          let mut tid2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[tid1, 9]));
          let mut idValue4: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[tid1, 1]));
          num30 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue4, 3)));
          let mut x2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[tid1, 3]));
          let mut y4: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[tid1, 4]));
          integer1 = Conversions.ToInteger(self.game.EventRelatedObj.CheckLibVarHex(x2, y4, libName1, "Zones"));
          let mut num35: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[tid1, 0]));
          if (se1AssetZone > 0 | x2 == self.game.SelectX & y4 == self.game.SelectY && num35 == se1AssetZone | integer1 == se1AssetZone &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue4, 5))) == num29)
          {
            let mut num36: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue4, 27)));
            if (num36 >= 0 & num36 <= 9)
            {
              int[] numArray2 = numArray1;
              int[] numArray3 = numArray2;
              let mut index4: i32 = num36;
              let mut index5: i32 = index4;
              let mut num37: i32 = numArray2[index4] + 1;
              numArray3[index5] = num37;
            }
            if (integer1 == num35 | self.game.EditObj.se1_AssetCategory1 == 2)
            {
              if (num29 == 1)
                num31 += 1;
              if (num29 == 0)
                num32 += 1;
              if (num35 != integer1)
                num33 += 1;
            }
            if (x2 == self.game.SelectX & y4 == self.game.SelectY)
            {
              num38: i32;
              num38 += 1;
            }
            num34 += 1;
            bool flag = true;
            if (self.game.EditObj.se1_AssetCategory2 == 1 & num29 == 0)
              flag = false;
            if (self.game.EditObj.se1_AssetCategory2 == 2 & num29 == 1)
              flag = false;
            if (self.game.EditObj.se1_AssetCategory2 == 3 & num35 == integer1)
              flag = false;
            if (self.game.EditObj.se1_AssetCategory5 > 0 && num36 != self.game.EditObj.se1_AssetCategory5)
              flag = false;
            if (self.game.EditObj.se1_assetItemMode1 > 0 && self.listItemAsset.FindWeight(tid2, 4, self.game.EditObj.se1_assetItemMode1) < 1)
              flag = false;
            if (self.game.EditObj.se1_assetItemMode2 > 0 && self.listItemAsset.FindWeight(tid2, 5, self.game.EditObj.se1_assetItemMode2) < 1)
              flag = false;
            if (flag)
              simpleList1.Add(tid1, num29 * 100000 + x2 * 200 + y4);
          }
        }
        num29 += -1;
      }
      while (num29 >= 0);
      data1: DataClass = self.game.Data;
      str1: String = "perk";
       local1: String =  str1;
      libName2: String = libName1;
      let mut libVar: i32 = data1.FindLibVar( local1, libName2);
      data2: DataClass = self.game.Data;
      str2: String = "hexname";
       local2: String =  str2;
      libName3: String = libName1;
      data2.FindLibVar( local2, libName3);
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index6: i32 = 0; index6 <= mapWidth; index6 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index7: i32 = 0; index7 <= mapHeight; index7 += 1)
        {
          if (Conversions.ToInteger(self.game.EventRelatedObj.CheckLibVarHex(index6, index7, libName1, "Zones")) == se1AssetZone)
          {
            let mut hexLibVarValue: i32 = self.game.Data.MapObj[0].HexObj[index6, index7].GetHexLibVarValue(libVar);
            if (hexLibVarValue > 0)
            {
              num34 += 1;
              num32 += 1;
              int[] numArray4 = numArray1;
              int[] numArray5 = numArray4;
              let mut index8: i32 = 7;
              let mut index9: i32 = index8;
              let mut num39: i32 = numArray4[index8] + 1;
              numArray5[index9] = num39;
              if (self.game.EditObj.se1_AssetCategory2 != 1 && !(self.game.EditObj.se1_AssetCategory1 == 1 & !(index6 == self.game.SelectX & index7 == self.game.SelectY)) && self.game.EditObj.se1_AssetCategory5 < 1 | self.game.EditObj.se1_AssetCategory5 == 7)
              {
                bool flag = true;
                if (self.game.EditObj.se1_assetItemMode2 > 0 && self.listItemAsset.FindWeight(1000000 + hexLibVarValue, 5, self.game.EditObj.se1_assetItemMode2) < 1)
                  flag = false;
                if (self.game.EditObj.se1_assetItemMode1 > 0)
                  flag = false;
                if (flag)
                {
                  let mut num40: i32 = 1000 * index6 + index7;
                  simpleList1.Add(9000000 + num40, 5000, index6, index7, hexLibVarValue);
                }
              }
            }
          }
        }
      }
      simpleList1.ReverseSort();
      let mut selectAssetButton: i32 = self.game.EditObj.se1_SelectAssetButton;
      let mut num41: i32 = 0;
      let mut num42: i32 = -1;
      let mut id4: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZones].GetData(0, self.game.EditObj.se1_assetZone, 6)));
      let mut num43: i32 = -1;
      let mut num44: i32 = -1;
      if (id4 > 0)
      {
        let mut locationById: i32 = self.game.HandyFunctionsObj.GetLocationByID(id4);
        if (locationById > -1)
        {
          num43 = self.game.Data.LocObj[locationById].X;
          num44 = self.game.Data.LocObj[locationById].Y;
        }
        else
          num13 = 0;
      }
      let mut num45: i32 = 1;
      num46: i32;
      do
      {
        let mut counter4: i32 = simpleList1.Counter;
        for (let mut index10: i32 = 0; index10 <= counter4; index10 += 1)
        {
          let mut index11: i32 = simpleList1.Id[index10];
          let mut num47: i32 = -1;
          let mut num48: i32 = 0;
          num46 = -1;
          idValue5: i32;
          num49: i32;
          num50: i32;
          num51: i32;
          if (index11 >= 9000000 & index11 < 15000000)
          {
            num47 = simpleList1.Data3[index10];
            let mut num52: i32 = simpleList1.Data1[index10];
            let mut num53: i32 = simpleList1.Data2[index10];
            idValue5 = -1;
            num49 = 9000000 + num52 * 1000 + num53;
            num50 = se1AssetZone;
            if (num10 == num49)
            {
              num28 = num52;
              num51 = num53;
              num10 = num49;
            }
          }
          else if (index11 >= 15000000 & index11 < 16000000)
          {
            let mut num54: i32 = simpleList1.Data1[index10];
            let mut num55: i32 = simpleList1.Data2[index10];
            idValue5 = -1;
            num49 = 15000000 + num54 * 1000 + num55;
            num50 = se1AssetZone;
            num48 = simpleList1.Data3[index10];
            if (num10 == num49)
            {
              num28 = num54;
              num51 = num55;
              num10 = num49;
            }
          }
          else
          {
            num47 = -1;
            num49 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index11, 9]));
            idValue5 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index11, 1]));
            num30 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue5, 3)));
            let mut x3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index11, 3]));
            let mut y5: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index11, 4]));
            integer1 = Conversions.ToInteger(self.game.EventRelatedObj.CheckLibVarHex(x3, y5, libName1, "Zones"));
            num50 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index11, 0]));
            bool flag = false;
            if (num10 == num49)
            {
              num28 = x3;
              num51 = y5;
              num10 = num49;
              flag = true;
            }
          }
          if (num50 == se1AssetZone | integer1 == se1AssetZone &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue5, 5))) == num45 | num47 > 0 & num45 == 0 | num48 > 0 & num45 == 0)
          {
            num41 += 1;
            if (num42 == -1 & num49 == num10)
              num42 = num41;
          }
        }
        num45 += -1;
      }
      while (num45 >= 0);
      let mut num56: i32 = 0;
      let mut num57: i32 =  Math.Round(Math.Floor( simpleList1.Counter /  num26)) + 1;
      num58: i32;
      if (num42 > -1)
      {
        num56 =  Math.Round(Math.Floor( (num42 - 1) /  num26));
        num58 = num56 * num26 + 1;
      }
      else
        num58 = 1;
      if (self.game.EditObj.se1_assetPage2 > num57 | self.game.EditObj.se1_assetPage2 < 1)
      {
        self.game.EditObj.se1_assetPage2 = num56 + 1;
        self.prevAssetId = -1;
      }
      if (self.prevAssetId == num10)
      {
        num58 = (self.game.EditObj.se1_assetPage2 - 1) * num26 + 1;
        if (num58 < 1)
          num58 = 1;
      }
      else
        self.game.EditObj.se1_assetPage2 = num56 + 1;
      self.prevAssetId = num10;
      let mut num59: i32 = num57;
      if (num59 > 8)
        num59 = 8;
      let mut num60: i32 =  Math.Round(Math.Floor( rectangle6.Width /  num59)) - 4;
      if (num60 > 100)
        num60 = 100;
      let mut x4: i32 = rectangle2.X +  Math.Round( rectangle2.Width / 2.0) -  Math.Round( (num59 * (num60 + 4)) / 2.0);
      let mut y6: i32 = rectangle2.Y;
      let mut num61: i32 = 1;
      if (num59 < num57)
      {
        num61 = self.game.EditObj.se1_assetPage2;
        if (num61 > num57 - 4)
          num61 = num57 - 4;
        if (num61 > self.game.EditObj.se1_assetPage2 - 3)
          num61 = self.game.EditObj.se1_assetPage2 - 3;
        if (1 > num61)
          num61 = 1;
      }
      let mut num62: i32 = num59;
      for (let mut index12: i32 = 1; index12 <= num62; index12 += 1)
      {
        let mut num63: i32 = num61 - 1 + index12;
        if (num63 >= 1 & num63 <= num57)
        {
          this += 1.assetButtonCounter;
          tDescript: String = num63.ToString() + "/" + num57.ToString() + ". Click to view this Asset page.";
          if (self.game.EditObj.se1_assetPage2 == num63)
            tDescript = num63.ToString() + "/" + num57.ToString() + ". Currently selected Asset page for this Zone";
          int[] assetButton = self.assetButton;
          let mut assetButtonCounter2: i32 = self.assetButtonCounter;
          let mut tsubpart9: SubPartClass =  new SEBigTextPartClass(num63.ToString(), tDescript, self.game.EditObj.se1_assetPage2 == num63, num60, 44);
          let mut num64: i32 = self.AddSubPart( tsubpart9, x4, y6, num60, 44, 1);
          assetButton[assetButtonCounter2] = num64;
          self.assetButtonData[self.assetButtonCounter] = 50 + num63;
          x4 += num60 + 4;
        }
      }
      let mut x5: i32 = rectangle9.X + 5;
      let mut y7: i32 = rectangle9.Y;
      this += 1.assetButtonCounter;
      tDataString1: String = num31.ToString();
      tDescript1: String = "Only Public Assets are shown if this button is tapped.";
      int[] assetButton1 = self.assetButton;
      let mut assetButtonCounter3: i32 = self.assetButtonCounter;
      let mut tsubpart10: SubPartClass =  new SEZoneButtonShortPartClass(18, -1, tDataString1, tDescript1, self.game.EditObj.se1_AssetCategory2 == 1);
      let mut num65: i32 = self.AddSubPart( tsubpart10, x5, y7, 93, 40, 1);
      assetButton1[assetButtonCounter3] = num65;
      self.assetButtonData[self.assetButtonCounter] = 13;
      let mut x6: i32 = x5 + 97;
      this += 1.assetButtonCounter;
      tDataString2: String = num32.ToString();
      tDescript2: String = "Only Private Assets are shown if this button is tapped.";
      int[] assetButton2 = self.assetButton;
      let mut assetButtonCounter4: i32 = self.assetButtonCounter;
      let mut tsubpart11: SubPartClass =  new SEZoneButtonShortPartClass(19, -1, tDataString2, tDescript2, self.game.EditObj.se1_AssetCategory2 == 2);
      let mut num66: i32 = self.AddSubPart( tsubpart11, x6, y7, 93, 40, 1);
      assetButton2[assetButtonCounter4] = num66;
      self.assetButtonData[self.assetButtonCounter] = 14;
      let mut y8: i32 = y7 + 41;
      let mut x7: i32 = rectangle9.X + 5;
      this += 1.assetButtonCounter;
      tDataString3: String = numArray1[1].ToString();
      tDescript3: String = "Only Agriculture Assets are shown if this button is tapped.";
      int[] assetButton3 = self.assetButton;
      let mut assetButtonCounter5: i32 = self.assetButtonCounter;
      let mut tsubpart12: SubPartClass =  new SEZoneButtonShortPartClass(58, -1, tDataString3, tDescript3, self.game.EditObj.se1_AssetCategory5 == 1);
      let mut num67: i32 = self.AddSubPart( tsubpart12, x7, y8, 93, 40, 1);
      assetButton3[assetButtonCounter5] = num67;
      self.assetButtonData[self.assetButtonCounter] = 161;
      let mut x8: i32 = x7 + 97;
      this += 1.assetButtonCounter;
      tDataString4: String = numArray1[2].ToString();
      tDescript4: String = "Only Mining Assets are shown if this button is tapped.";
      int[] assetButton4 = self.assetButton;
      let mut assetButtonCounter6: i32 = self.assetButtonCounter;
      let mut tsubpart13: SubPartClass =  new SEZoneButtonShortPartClass(59, -1, tDataString4, tDescript4, self.game.EditObj.se1_AssetCategory5 == 2);
      let mut num68: i32 = self.AddSubPart( tsubpart13, x8, y8, 93, 40, 1);
      assetButton4[assetButtonCounter6] = num68;
      self.assetButtonData[self.assetButtonCounter] = 162;
      let mut y9: i32 = y8 + 41;
      let mut x9: i32 = rectangle9.X + 5;
      this += 1.assetButtonCounter;
      tDataString5: String = numArray1[3].ToString();
      tDescript5: String = "Only Energy Assets are shown if this button is tapped.";
      int[] assetButton5 = self.assetButton;
      let mut assetButtonCounter7: i32 = self.assetButtonCounter;
      let mut tsubpart14: SubPartClass =  new SEZoneButtonShortPartClass(32, -1, tDataString5, tDescript5, self.game.EditObj.se1_AssetCategory5 == 3);
      let mut num69: i32 = self.AddSubPart( tsubpart14, x9, y9, 93, 40, 1);
      assetButton5[assetButtonCounter7] = num69;
      self.assetButtonData[self.assetButtonCounter] = 163;
      let mut x10: i32 = x9 + 97;
      this += 1.assetButtonCounter;
      tDataString6: String = numArray1[4].ToString();
      tDescript6: String = "Only Industry Assets are shown if this button is tapped.";
      int[] assetButton6 = self.assetButton;
      let mut assetButtonCounter8: i32 = self.assetButtonCounter;
      let mut tsubpart15: SubPartClass =  new SEZoneButtonShortPartClass(60, -1, tDataString6, tDescript6, self.game.EditObj.se1_AssetCategory5 == 4);
      let mut num70: i32 = self.AddSubPart( tsubpart15, x10, y9, 93, 40, 1);
      assetButton6[assetButtonCounter8] = num70;
      self.assetButtonData[self.assetButtonCounter] = 164;
      let mut y10: i32 = y9 + 41;
      let mut x11: i32 = rectangle9.X + 5;
      this += 1.assetButtonCounter;
      tDataString7: String = numArray1[5].ToString();
      tDescript7: String = "Only QOL / Research / Government Assets are shown if this button is tapped.";
      int[] assetButton7 = self.assetButton;
      let mut assetButtonCounter9: i32 = self.assetButtonCounter;
      let mut tsubpart16: SubPartClass =  new SEZoneButtonShortPartClass(17, -1, tDataString7, tDescript7, self.game.EditObj.se1_AssetCategory5 == 5);
      let mut num71: i32 = self.AddSubPart( tsubpart16, x11, y10, 93, 40, 1);
      assetButton7[assetButtonCounter9] = num71;
      self.assetButtonData[self.assetButtonCounter] = 165;
      let mut x12: i32 = x11 + 97;
      this += 1.assetButtonCounter;
      tDataString8: String = numArray1[6].ToString();
      tDescript8: String = "Only Logistical Assets are shown if this button is tapped.";
      int[] assetButton8 = self.assetButton;
      let mut assetButtonCounter10: i32 = self.assetButtonCounter;
      let mut tsubpart17: SubPartClass =  new SEZoneButtonShortPartClass(50, -1, tDataString8, tDescript8, self.game.EditObj.se1_AssetCategory5 == 6);
      let mut num72: i32 = self.AddSubPart( tsubpart17, x12, y10, 93, 40, 1);
      assetButton8[assetButtonCounter10] = num72;
      self.assetButtonData[self.assetButtonCounter] = 166;
      let mut y11: i32 = y10 + 41;
      let mut x13: i32 = rectangle9.X + 5;
      this += 1.assetButtonCounter;
      tDataString9: String = numArray1[7].ToString();
      tDescript9: String = "Only Hex Feats and Auxilliary Assets are shown if this button is tapped.";
      int[] assetButton9 = self.assetButton;
      let mut assetButtonCounter11: i32 = self.assetButtonCounter;
      let mut tsubpart18: SubPartClass =  new SEZoneButtonShortPartClass(1, -1, tDataString9, tDescript9, self.game.EditObj.se1_AssetCategory5 == 7);
      let mut num73: i32 = self.AddSubPart( tsubpart18, x13, y11, 93, 40, 1);
      assetButton9[assetButtonCounter11] = num73;
      self.assetButtonData[self.assetButtonCounter] = 167;
      let mut x14: i32 = x13 + 97;
      this += 1.assetButtonCounter;
      tDataString10: String = num33.ToString();
      tDescript10: String = "Delegated and Tasked Assets are shown if this button is tapped.";
      int[] assetButton10 = self.assetButton;
      let mut assetButtonCounter12: i32 = self.assetButtonCounter;
      let mut tsubpart19: SubPartClass =  new SEZoneButtonShortPartClass(39, -1, tDataString10, tDescript10, self.game.EditObj.se1_AssetCategory2 == 3);
      let mut num74: i32 = self.AddSubPart( tsubpart19, x14, y11, 93, 40, 1);
      assetButton10[assetButtonCounter12] = num74;
      self.assetButtonData[self.assetButtonCounter] = 15;
      let mut num75: i32 = y11 + 41;
      let mut num76: i32 = 0;
      num15 = -1;
      let mut num77: i32 = -1;
      num75 = 18;
      color: Color = Color.FromArgb(100,  byte.MaxValue,  byte.MaxValue, 0);
      if (turn > -1)
        color = Color.FromArgb(200, self.game.Data.RegimeObj[turn].Red, self.game.Data.RegimeObj[turn].Green, self.game.Data.RegimeObj[turn].Blue);
      let mut num78: i32 =  Math.Round( (rectangle6.Width -  Math.Round(Math.Floor( rectangle6.Width / 160.0)) * 160) / 2.0);
      let mut x15: i32 = rectangle6.X - 160 + num78;
      let mut y12: i32 = rectangle6.Y;
      let mut num79: i32 = 1;
      str3: String;
      Rectangle rectangle10;
      do
      {
        let mut counter5: i32 = simpleList1.Counter;
        for (let mut index13: i32 = 0; index13 <= counter5; index13 += 1)
        {
          let mut index14: i32 = simpleList1.Id[index13];
          let mut idValue6: i32 = -1;
          num46 = -1;
          let mut num80: i32 = 0;
          x16: i32;
          y13: i32;
          idValue7: i32;
          assetId: i32;
          num81: i32;
          idValue8: i32;
          num82: i32;
          regime1: i32;
          if (index14 >= 9000000 & index14 < 15000000)
          {
            idValue6 = simpleList1.Data3[index13];
            x16 = simpleList1.Data1[index13];
            y13 = simpleList1.Data2[index13];
            idValue7 = -1;
            assetId = 9000000 + x16 * 1000 + y13;
            num81 = se1AssetZone;
            idValue8 = se1AssetZone;
            num82 = 0;
            regime1 = self.game.Data.MapObj[0].HexObj[x16, y13].Regime;
          }
          else if (index14 >= 15000000 & index14 < 16000000)
          {
            num80 = simpleList1.Data3[index13];
            x16 = simpleList1.Data1[index13];
            y13 = simpleList1.Data2[index13];
            idValue7 = -1;
            assetId = 15000000 + x16 * 1000 + y13;
            num81 = se1AssetZone;
            idValue8 = se1AssetZone;
            num82 = 0;
            regime1 = self.game.Data.MapObj[0].HexObj[x16, y13].Regime;
          }
          else
          {
            idValue6 = -1;
            assetId =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 9]));
            idValue7 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 1]));
            num30 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 3)));
            x16 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 3]));
            y13 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 4]));
            num82 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 5)));
            idValue8 = Conversions.ToInteger(self.game.EventRelatedObj.CheckLibVarHex(x16, y13, libName1, "Zones"));
            regime1 = self.game.Data.MapObj[0].HexObj[x16, y13].Regime;
            num81 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 0]));
          }
          if (num81 == se1AssetZone | idValue8 == se1AssetZone && num80 > 0 & num79 == 0 | idValue6 > -1 & num79 == 0 |  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 5))) == num79)
          {
            num76 += 1;
            num25 += 1;
            if (num76 >= num58 & num76 < num58 + num26)
            {
              if (num10 < 1)
                num10 = assetId;
              if (num27 == -1)
                num27 = num76;
              num77 += 1;
              x15 += 160;
              if (x15 > rectangle6.Right - 160)
              {
                x15 = rectangle6.X + num78;
                y12 += 210;
              }
              if (self.game.EditObj.se1_SelectAssetButton < 1 & x16 == self.game.SelectX & y13 == self.game.SelectY)
                self.game.EditObj.se1_SelectAssetButton = assetId;
              customBitmapObj: CustomBitmapClass = self.game.CustomBitmapObj;
               let mut local3: &Graphics = &g;
              let mut tx: i32 = x15;
              let mut ty: i32 = y12;
              WindowClass windowClass = (WindowClass) this;
               WindowClass local4 =  windowClass;
              let mut curAssetId: i32 = num10;
              let mut assetRowOrSpecialCode: i32 = index14;
              let mut specialOnX: i32 = x16;
              let mut specialOnY: i32 = y13;
              let mut specialType: i32 = simpleList1.Data3[index13];
              let mut zoneNr: i32 = se1AssetZone;
              let mut zoneRegNr: i32 = turn;
              let mut num83: i32 = self.game.EditObj.se1_assetMode == 2 ? 1 : 0;
              customBitmapObj.Se1_DrawAssetBlock( local3, tx, ty,  local4, curAssetId, assetRowOrSpecialCode, specialOnX, specialOnY, specialType, zoneNr, zoneRegNr, num83 != 0);
              str4: String;
              Rectangle rectangle11;
              str5: String;
              if (idValue7 > 0 & self.game.EditObj.se1_SelectAssetButton == assetId)
              {
                let mut num84: i32 = x15;
                let mut num85: i32 = y12;
                let mut regime2: i32 = self.game.Data.MapObj[0].HexObj[x16, y13].Regime;
                let mut id5: i32 = self.game.Data.RegimeObj[regime2].id;
                let mut num86: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 25)));
                let mut num87: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 5)));
                let mut num88: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 13]));
                let mut num89: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 11]));
                let mut num90: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 6]));
                let mut num91: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 15]));
                let mut num92: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 8]));
                let mut num93: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 2)));
                str3 = self.game.Data.StringListObj[self.slotAssets].Data[index14, 10];
                if (self.game.Data.MapObj[0].HexObj[x16, y13].Location > -1 & idValue8 > 0 &&  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotZones].GetData(0, idValue8, 6))) != self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x16, y13].Location].ID)
                {
                  name: String = self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x16, y13].Location].Name;
                  self.game.Data.StringListObj[self.slotAssets].Data[index14, 10] = name;
                }
                str4 = self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 1);
                str6: String = self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 12);
                let mut num94: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 5]));
                let mut num95: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 4)));
                let mut num96: i32 = rectangle5.X + 20;
                let mut y14: i32 = rectangle5.Y;
                let mut w1: i32 = 220;
                let mut h1: i32 = 150;
                DrawMod.DrawBlock( g, num96, y14, w1, h1, 0, 0, 0, 100);
                DrawMod.DrawBlock( g, num96, y14, w1, 40, 0, 0, 0, 100);
                let mut index15: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 8)));
                let mut nr: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 2)));
                tstring2: String = self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 1);
                data3: String = self.game.Data.StringListObj[self.slotAssetTypes].GetData(0, idValue7, 12);
                if (nr > 0)
                {
                  str6 = data3 + " " + self.game.HandyFunctionsObj.GetRomanNumerical(nr);
                  tstring2 = tstring2 + " " + self.game.HandyFunctionsObj.GetRomanNumerical(nr);
                }
                if (index15 > 0)
                {
                  num60 = nr;
                  if (num60 > 5)
                    num60 = 5;
                  if (num60 < 1)
                    num60 = 1;
                  --num60;
                  let mut x17: i32 = 2 + num60 * 134;
                   let mut local5: &Graphics = &g;
                  bitmap: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[index15]);
                   let mut local6: &Bitmap = &bitmap;
                  rectangle11 = Rectangle::new(x17, 2, 131, 111);
                  let mut srcrect: &Rectangle = &rectangle11
                  rectangle10 = Rectangle::new(num96, y14 + 40, 131, 111);
                  let mut destrect: &Rectangle = &rectangle10
                  DrawMod.DrawSimplePart2( local5,  local6, srcrect, destrect);
                  DrawMod.DrawTextColouredMarcCenter( g, tstring2, self.game.MarcFont4, num96 + 110, y14 + 10, Color.White);
                }
                let mut x18: i32 = num96 + 130;
                let mut y15: i32 = y14 + 45;
                c: Color = Color.FromArgb( byte.MaxValue, 180, 180, 180);
                let mut num97: i32 = 16;
                let mut idValue9: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 0]));
                str5 = idValue8 == se1AssetZone ? (num81 != se1AssetZone ? "DEL.TO:" + self.game.Data.StringListObj[self.slotZones].GetData(0, idValue9, 7) : (se1AssetZone >= 1 ? "ZONE:" + self.game.Data.StringListObj[self.slotZones].GetData(0, idValue9, 7) : "Hex without zone")) : (regime2 == self.game.Data.Turn ? "TASK FROM:" + self.game.Data.StringListObj[self.slotZones].GetData(0, idValue8, 7) : "ZONE:Evacuated Asset");
                if (num92 > 0)
                {
                  y16: i32;
                  if (turn == self.game.Data.Turn)
                  {
                    tstring3: String = "UPKP: " + num88.ToString() + "%";
                    DrawMod.DrawTextColouredConsole( g, tstring3, DrawMod.TGame.se1TypeWriterMedium, x18, y15, DrawMod.TGame.seColTW);
                    let mut y17: i32 = y15 + num97;
                    tstring4: String = "CONS: " + num89.ToString() + "%";
                    DrawMod.DrawTextColouredConsole( g, tstring4, DrawMod.TGame.se1TypeWriterMedium, x18, y17, DrawMod.TGame.seColTW);
                    y16 = y17 + num97;
                  }
                  else
                  {
                    tstring5: String = "CONS: " + num89.ToString() + "%";
                    DrawMod.DrawTextColouredConsole( g, tstring5, DrawMod.TGame.se1TypeWriterMedium, x18, y15, DrawMod.TGame.seColTW);
                    y16 = y15 + num97;
                  }
                  tstring6: String = "DAM: " + num90.ToString() + " pts";
                  DrawMod.DrawTextColouredMarc( g, tstring6, self.game.MarcFont4, x18, y16, c);
                  let mut y18: i32 = y16 + num97;
                  if (turn == self.game.Data.Turn)
                  {
                    Left: String = ( Math.Round( ( Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 7])) * 100 -  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotAssets].Data[index14, 12]))) / 100.0, 1)).ToString();
                    if (Operators.CompareString(Left, "0", false) == 0)
                      Left = "0.1";
                    tstring7: String = "CONS: " + Left + "t.";
                    DrawMod.DrawTextColouredMarc( g, tstring7, self.game.MarcFont4, x18, y18, c);
                    num75 = y18 + num97;
                  }
                }
                else
                {
                  if (turn == self.game.Data.Turn)
                  {
                    tstring8: String = "UPKP: " + num88.ToString() + "%";
                    DrawMod.DrawTextColouredMarc( g, tstring8, self.game.MarcFont4, x18, y15, c);
                    let mut y19: i32 = y15 + num97;
                    tstring9: String = "PROD: " + num89.ToString() + "%";
                    DrawMod.DrawTextColouredMarc( g, tstring9, self.game.MarcFont4, x18, y19, c);
                    y15 = y19 + num97;
                  }
                  tstring10: String = "DAM: " + num90.ToString();
                  DrawMod.DrawTextColouredMarc( g, tstring10, self.game.MarcFont4, x18, y15, c);
                  let mut y20: i32 = y15 + num97;
                  if (turn == self.game.Data.Turn)
                  {
                    tstring11: String = !(num91 > 0 & num91 < 100) ? "LIM: 100%" : "LIM: " + num91.ToString() + "%";
                    DrawMod.DrawTextColouredMarc( g, tstring11, self.game.MarcFont4, x18, y20, c);
                    y20 += num97;
                  }
                  if (turn == self.game.Data.Turn && x16 > -1 & num95 == 5)
                  {
                    let mut location: i32 = self.game.Data.MapObj[0].HexObj[x16, y13].Location;
                    let mut num98: i32 = 0;
                    let mut num99: i32 = 0;
                    if (location > -1)
                    {
                      let mut logCounter: i32 = self.game.Data.LocObj[location].LogCounter;
                      for (let mut index16: i32 = 0; index16 <= logCounter; index16 += 1)
                      {
                        if (self.game.Data.LocObj[location].LogType[index16] >= 801 & self.game.Data.LocObj[location].LogType[index16] <= 899)
                          num98 += self.game.Data.LocObj[location].LogData3[index16];
                        if (self.game.Data.LocObj[location].LogType[index16] >= 901 & self.game.Data.LocObj[location].LogType[index16] <= 999)
                          num99 += self.game.Data.LocObj[location].LogData3[index16];
                      }
                      if (num98 > 0 | num99 > 0)
                      {
                        tstring12: String = "L.EXT: " + num99.ToString();
                        DrawMod.DrawTextColouredMarc( g, tstring12, self.game.MarcFont4, x18, y20, c);
                        let mut y21: i32 = y20 + num97;
                        tstring13: String = "NXT: " + num98.ToString();
                        DrawMod.DrawTextColouredMarc( g, tstring13, self.game.MarcFont4, x18, y21, c);
                        num75 = y21 + num97;
                      }
                    }
                  }
                }
                let mut x1_1: i32 = rectangle5.X + 20 + 220 + 10;
                let mut y22: i32 = rectangle5.Y;
                let mut w2: i32 = 330;
                let mut h2: i32 = 150;
                DrawMod.DrawBlock( g, x1_1, y22, w2, h2, 0, 0, 0, 100);
                DrawMod.DrawBlock( g, x1_1, y22, w2, 40, 0, 0, 0, 100);
                DrawMod.DrawTextColouredMarcCenter( g, "Options", self.game.MarcFont4, x1_1 + 165, y22 + 10, Color.White);
                if (turn == self.game.Data.Turn)
                {
                  let mut num100: i32 = y22 + 45;
                  let mut num101: i32 = x1_1 + 15;
                  let mut num102: i32 = 90;
                  if (num82 < 1)
                    num102 = 160;
                  let mut num103: i32 = num100;
                  SubPartClass tsubpart20;
                  if (num82 < 1)
                  {
                    let mut num104: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotRegKey].GetData2(0, self.game.Data.RegimeObj[turn].id, 1, "credits", 2)));
                    num60 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotConstruction].GetData3(0, idValue7, 1, 3, 2, "credits", 3)));
                    buttontext: String = "Nationalize [" + num60.ToString() + "Cr]";
                    tDescript11: String;
                    num105: i32;
                    if (num60 > num104)
                    {
                      tDescript11 = "You do not have the " + num60.ToString() + " credits required to nationalize this asset. ";
                      num105 = 1;
                    }
                    else
                    {
                      tDescript11 = "Nationalizing this asset will cost you " + num60.ToString() + " credits. ";
                      num105 = 0;
                    }
                    if (num86 < 1)
                    {
                      tDescript11 = "This Asset Type cannot be nationalized. ";
                      num105 = 1;
                      buttontext = "Nationalize";
                    }
                    else if (num92 > 0)
                    {
                      tDescript11 = "A Private Asset in construction cannot be nationalized. ";
                      num105 = 1;
                      buttontext = "Nationalize";
                    }
                    else if (self.game.EventRelatedObj.Helper_IsAssetUnderConstructionOrUpgrade(assetId))
                    {
                      tDescript11 = "A Private Asset in the process of being upgraded cannot be nationalized. ";
                      num105 = 1;
                      buttontext = "Nationalize";
                    }
                    this += 1.assetButtonCounter;
                    int[] assetButton11 = self.assetButton;
                    let mut assetButtonCounter13: i32 = self.assetButtonCounter;
                    tsubpart20 =  new TextButtonPartClass(buttontext, num102, tDescript11,  self.OwnBitmap, num101, num100, num105 == 1, theight: 25, usefont: DrawMod.TGame.MarcFont4, useshadow: true, tMarcStyle: true);
                    let mut num106: i32 = self.AddSubPart( tsubpart20, num101, num100, num102, 25, 1);
                    assetButton11[assetButtonCounter13] = num106;
                    self.assetButtonData[self.assetButtonCounter] = 21;
                    if (num105 == 1)
                      self.assetButtonData[self.assetButtonCounter] = 0;
                    num100 += 25;
                  }
                  tDescript12: String = "Change the zone this asset is delegated to";
                  this += 1.assetButtonCounter;
                  let mut num107: i32 = 0;
                  if (x16 == num43 & y13 == num44)
                  {
                    num107 = 1;
                    tDescript12 = "Cannot Delegate Assets in the City, only in Rural Hexes.";
                  }
                  if (num82 < 1 && num81 == se1AssetZone)
                  {
                    num107 = 1;
                    tDescript12 = "Cannot Delegate Private Assets, only Public ones.";
                  }
                  if (num81 != se1AssetZone)
                  {
                    int[] assetButton12 = self.assetButton;
                    let mut assetButtonCounter14: i32 = self.assetButtonCounter;
                    tsubpart20 =  new TextButtonPartClass("(Un)delegate", num102, tDescript12,  self.OwnBitmap, num101, num100, num107 == 1, theight: 25, usefont: DrawMod.TGame.MarcFont4, useshadow: true, tMarcStyle: true);
                    let mut num108: i32 = self.AddSubPart( tsubpart20, num101, num100, num102, 25, 1);
                    assetButton12[assetButtonCounter14] = num108;
                  }
                  else
                  {
                    int[] assetButton13 = self.assetButton;
                    let mut assetButtonCounter15: i32 = self.assetButtonCounter;
                    tsubpart20 =  new TextButtonPartClass("Delegate", num102, tDescript12,  self.OwnBitmap, num101, num100, num107 == 1, theight: 25, usefont: DrawMod.TGame.MarcFont4, useshadow: true, tMarcStyle: true);
                    let mut num109: i32 = self.AddSubPart( tsubpart20, num101, num100, num102, 25, 1);
                    assetButton13[assetButtonCounter15] = num109;
                  }
                  self.assetButtonData[self.assetButtonCounter] = 22;
                  if (num107 == 1)
                    self.assetButtonData[self.assetButtonCounter] = 0;
                  let mut num110: i32 = num103;
                  let mut num111: i32 = num101 + 100;
                  let mut num112: i32 = 90;
                  if (num87 == 1)
                  {
                    if (num92 == 1)
                    {
                      let mut num113: i32 = 0;
                      tDescript13: String = "Cancel Construction";
                      this += 1.assetButtonCounter;
                      int[] assetButton14 = self.assetButton;
                      let mut assetButtonCounter16: i32 = self.assetButtonCounter;
                      tsubpart20 =  new TextButtonPartClass("Cancel Constr.", num112, tDescript13,  self.OwnBitmap, num111, num110, num113 == 1, num113 == 1, 25, usefont: DrawMod.TGame.MarcFont4, useshadow: true, tMarcStyle: true);
                      let mut num114: i32 = self.AddSubPart( tsubpart20, num111, num110, num112, 25, 1);
                      assetButton14[assetButtonCounter16] = num114;
                      self.assetButtonData[self.assetButtonCounter] = 23;
                      if (num113 == 1)
                        self.assetButtonData[self.assetButtonCounter] = 0;
                      num75 = num110 + 25;
                    }
                    else
                    {
                      let mut num115: i32 = 1;
                      if (num94 < 0)
                        num115 = 0;
                      tDescript14: String = "Set Asset to Active Mode";
                      this += 1.assetButtonCounter;
                      int[] assetButton15 = self.assetButton;
                      let mut assetButtonCounter17: i32 = self.assetButtonCounter;
                      tsubpart20 =  new TextButtonPartClass("Active", num112, tDescript14,  self.OwnBitmap, num111, num110, num115 == 1, num115 == 1, 25, usefont: DrawMod.TGame.MarcFont4, useshadow: true, tMarcStyle: true);
                      let mut num116: i32 = self.AddSubPart( tsubpart20, num111, num110, num112, 25, 1);
                      assetButton15[assetButtonCounter17] = num116;
                      self.assetButtonData[self.assetButtonCounter] = 31;
                      if (num115 == 1)
                        self.assetButtonData[self.assetButtonCounter] = 0;
                      let mut num117: i32 = num110 + 25;
                      let mut num118: i32 = 1;
                      if (num94 != -1)
                        num118 = 0;
                      tDescript15: String = "Set Asset to Mothball Mode";
                      this += 1.assetButtonCounter;
                      int[] assetButton16 = self.assetButton;
                      let mut assetButtonCounter18: i32 = self.assetButtonCounter;
                      tsubpart20 =  new TextButtonPartClass("Mothball", num112, tDescript15,  self.OwnBitmap, num111, num117, num118 == 1, num118 == 1, 25, usefont: DrawMod.TGame.MarcFont4, useshadow: true, tMarcStyle: true);
                      let mut num119: i32 = self.AddSubPart( tsubpart20, num111, num117, num112, 25, 1);
                      assetButton16[assetButtonCounter18] = num119;
                      self.assetButtonData[self.assetButtonCounter] = 32;
                      if (num118 == 1)
                        self.assetButtonData[self.assetButtonCounter] = 0;
                      let mut num120: i32 = num117 + 25;
                      let mut num121: i32 = 1;
                      if (num94 != -2)
                        num121 = 0;
                      tDescript16: String = "Close down the Asset";
                      this += 1.assetButtonCounter;
                      int[] assetButton17 = self.assetButton;
                      let mut assetButtonCounter19: i32 = self.assetButtonCounter;
                      tsubpart20 =  new TextButtonPartClass("Close", num112, tDescript16,  self.OwnBitmap, num111, num120, num121 == 1, num121 == 1, 25, usefont: DrawMod.TGame.MarcFont4, useshadow: true, tMarcStyle: true);
                      let mut num122: i32 = self.AddSubPart( tsubpart20, num111, num120, num112, 25, 1);
                      assetButton17[assetButtonCounter19] = num122;
                      self.assetButtonData[self.assetButtonCounter] = 33;
                      if (num121 == 1)
                        self.assetButtonData[self.assetButtonCounter] = 0;
                      num75 = num120 + 25;
                    }
                  }
                  if (num82 > 0)
                  {
                    let mut num123: i32 = num103;
                    let mut num124: i32 = num111 + 110;
                    let mut num125: i32 = 90;
                    let mut num126: i32 = 1;
                    do
                    {
                      this += 1.assetButtonCounter;
                      if (num126 == 1)
                        num60 = 100;
                      if (num126 == 2)
                        num60 = 75;
                      if (num126 == 3)
                        num60 = 50;
                      if (num126 == 4)
                        num60 = 25;
                      buttontext: String;
                      tDescript17: String;
                      if (num92 > 0)
                      {
                        buttontext = num60.ToString() + "% Cons";
                        tDescript17 = "Set maximum construction speed of Asset to " + buttontext + ".";
                      }
                      else
                      {
                        buttontext = num60.ToString() + "% Prod";
                        tDescript17 = "Set maximum production speed of Asset to " + buttontext + ".";
                      }
                      let mut num127: i32 = 0;
                      if (num91 == num60 | num91 == 0 & num60 == 100)
                        num127 = 1;
                      int[] assetButton18 = self.assetButton;
                      let mut assetButtonCounter20: i32 = self.assetButtonCounter;
                      tsubpart20 =  new TextButtonPartClass(buttontext, num125, tDescript17,  self.OwnBitmap, num124, num123, num127 == 1, num127 == 1, 25, usefont: DrawMod.TGame.MarcFont4, useshadow: true, tMarcStyle: true);
                      let mut num128: i32 = self.AddSubPart( tsubpart20, num124, num123, num125, 25, 1);
                      assetButton18[assetButtonCounter20] = num128;
                      self.assetButtonData[self.assetButtonCounter] = 2000 + num60;
                      if (num127 == 1)
                        self.assetButtonData[self.assetButtonCounter] = 0;
                      num123 += 25;
                      num126 += 1;
                    }
                    while (num126 <= 4);
                  }
                }
                x15 = num84;
                y12 = num85;
              }
              else if (idValue6 > 0)
              {
                if (num10 == assetId)
                {
                  let mut num129: i32 = x15;
                  let mut num130: i32 = y12;
                  let mut num131: i32 = rectangle5.X + 20;
                  let mut y23: i32 = rectangle5.Y;
                  let mut w: i32 = 220;
                  let mut h: i32 = 150;
                  DrawMod.DrawBlock( g, num131, y23, w, h, 0, 0, 0, 100);
                  DrawMod.DrawBlock( g, num131, y23, w, 40, 0, 0, 0, 100);
                  let mut integer2: i32 = Conversions.ToInteger(self.game.Data.StringListObj[self.slotPerks].GetData(0, idValue6, 13));
                  let mut num132: i32 = 0;
                  str5 = "";
                  str4 = "";
                  str3 = "";
                  data4: String = self.game.Data.StringListObj[self.slotPerks].GetData(0, idValue6, 1);
                  str3 = self.game.Data.StringListObj[self.slotPerks].GetData(0, idValue6, 7) + "\r\n\r\n" + "A Hex Perk is completely independent from your Public and Private Economy and delivers as long as it is connected to the Zone City.";
                  if (integer2 > 0)
                  {
                    num60 = num132;
                    if (num60 > 5)
                      num60 = 5;
                    if (num60 < 1)
                      num60 = 1;
                    --num60;
                    let mut x19: i32 = 2 + num60 * 134;
                     let mut local7: &Graphics = &g;
                    bitmap: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[integer2]);
                     let mut local8: &Bitmap = &bitmap;
                    rectangle10 = Rectangle::new(x19, 2, 131, 111);
                    let mut srcrect: &Rectangle = &rectangle10
                    rectangle11 = Rectangle::new(num131, y23 + 40, 131, 111);
                    let mut destrect: &Rectangle = &rectangle11
                    DrawMod.DrawSimplePart2( local7,  local8, srcrect, destrect);
                    DrawMod.DrawTextColouredMarcCenter( g, data4, self.game.MarcFont4, num131 + 110, y23 + 10, Color.White);
                  }
                  let mut x20: i32 = num131 + 130;
                  let mut y24: i32 = y23 + 45;
                  c: Color = Color.FromArgb( byte.MaxValue, 180, 180, 180);
                  let mut num133: i32 = 16;
                  tstring14: String = "Hex Perk.\r\nNo settings\r\npossible.";
                  DrawMod.DrawTextColouredMarc( g, tstring14, self.game.MarcFont4, x20, y24, c);
                  num75 = y24 + num133;
                  x15 = num129;
                  y12 = num130;
                }
              }
              else if (num80 > 0 && num10 == assetId)
              {
                tstring15: String = idValue8 >= 1 ? "ZONE:" + self.game.Data.StringListObj[self.slotZones].GetData(0, idValue8, 7) : "Hex without zone";
                DrawMod.DrawTextColouredConsole( g, tstring15, DrawMod.TGame.se1TypeWriterMedium, x15, y12, DrawMod.TGame.seColTW);
                let mut y25: i32 = y12 + 20;
                tstring16: String = "A Free Folk settlement.";
                DrawMod.DrawTextColouredConsole( g, tstring16, DrawMod.TGame.se1TypeWriterMedium, x15, y25, DrawMod.TGame.seColTW);
                let mut y26: i32 = y25 + 20;
                tstring17: String = "No settings possible.";
                DrawMod.DrawTextColouredConsole( g, tstring17, DrawMod.TGame.se1TypeWriterMedium, x15, y26, DrawMod.TGame.seColTW);
                y12 = y26 + 20;
              }
            }
          }
        }
        num79 += -1;
      }
      while (num79 >= 0);
      bool flag1 = false;
      if (self.game.EditObj.se1_assetMode > 1)
        flag1 = true;
      if (self.game.EditObj.se1_assetRightPanel > 0)
      {
        SimpleList simpleList2 = SimpleList::new();
        SimpleStringList simpleStringList = SimpleStringList::new();
        simpleList2.Add(7, 0, 19);
        simpleStringList.Add("Food\r\nYour workers and soldiers need food in order not to starve.", 1);
        simpleList2.Add(5, 0, 20);
        simpleStringList.Add("Water\r\nYour domed farms need water in order to produce food.", 1);
        simpleList2.Add(1, 0, 18);
        simpleStringList.Add("Oil\r\nNeeded to keep your mechanized troops mobile.", 1);
        simpleList2.Add(10, 0, 17);
        simpleStringList.Add("Ammo\r\nNeeded to keep your army fighting. Build up reserves before starting a war.", 1);
        simpleList2.Add(2, 0);
        simpleStringList.Add("Met\r\nBase resource needed to build almost anything. Includes iron, copper and nickel.", 1);
        simpleList2.Add(8, 0, 22);
        simpleStringList.Add("IP\r\nKey to producing almost anything.", 1);
        simpleList2.Add(15, 0, 16);
        simpleStringList.Add("Energy\r\nSome assets require energy in-order to run. Energy can be produced in power plants.", 1);
        simpleList2.Add(4, 0);
        simpleStringList.Add("Radio\r\nSome models will require Radioactives for construction and/or ammunitions.", 1);
        simpleList2.Add(9, 0);
        simpleStringList.Add("Recruit\r\nIn order to raise new troops you need recruits.", 1);
        simpleList2.Add(12, 0);
        simpleStringList.Add("Colonist\r\nTo found a new city or increase populace of a zone you need colonists.", 1);
        simpleList2.Add(3, 0);
        simpleStringList.Add("Rare\r\nIncludes rare earth metals. For advanced production.", 1);
        simpleList2.Add(13, 0);
        simpleStringList.Add("Machine\r\nFor construction of advanced equipment and assets.", 1);
        simpleList2.Add(14, 0);
        simpleStringList.Add("Hi-Tech\r\nFor construction of very advanced equipment and assets.", 1);
        self.game.Data.FindEventPic("", 9, "SE_Present");
        self.game.Data.FindEventPic("", 8, "SE_Present");
        self.game.Data.FindEventPic("", 11, "SE_Present");
        let mut num134: i32 = rectangle4.X + 5;
        let mut num135: i32 = rectangle4.Y + 5;
        let mut height2: i32 = 28;
        let mut num136: i32 = num134;
        let mut x21: i32 = num134 + 85;
        let mut num137: i32 = 1;
        do
        {
          tstring18: String;
          tstring19: String;
          if (!flag1)
          {
            if (num137 == 1)
              tstring18 = "";
            if (num137 == 2)
            {
              tstring18 = "DELI";
              tstring19 = "SHQ";
            }
            if (num137 == 3)
              tstring18 = "";
            if (num137 == 4)
            {
              tstring18 = "CONS";
              tstring19 = "ZONE";
            }
            if (num137 == 5)
            {
              tstring18 = "PROD";
              tstring19 = "ZONE";
            }
            if (num137 == 6)
              tstring18 = "";
            if (num137 == 7)
            {
              tstring18 = "PICK";
              tstring19 = "SHQ";
            }
            if (num137 == 8)
            {
              tstring18 = "Inv.";
              tstring19 = "ZONE";
            }
          }
          else
          {
            if (num137 == 1)
            {
              tstring18 = "Inv.";
              tstring19 = "Shq";
            }
            if (num137 == 2)
            {
              tstring18 = "DELV";
              tstring19 = "SHQ";
            }
            if (num137 == 3)
            {
              tstring18 = "Inv.";
              tstring19 = "Zone";
            }
            if (num137 == 4)
            {
              tstring18 = "CONS";
              tstring19 = "ZONE";
            }
            if (num137 == 5)
            {
              tstring18 = "PROD";
              tstring19 = "ZONE";
            }
            if (num137 == 6)
            {
              tstring18 = "Inv.";
              tstring19 = "Zone";
            }
            if (num137 == 7)
            {
              tstring18 = "PICK";
              tstring19 = "SHQ";
            }
            if (num137 == 8)
            {
              tstring18 = "Next";
              tstring19 = "Shq";
            }
          }
          if (tstring18.Length > 1)
          {
            DrawMod.DrawTextColouredConsole( g, tstring19, self.game.MarcFont16, x21, num135 + 4, self.game.seColGray);
            DrawMod.DrawTextColouredConsole( g, tstring18, self.game.MarcFont16, x21, num135 + 20, self.game.seColGray);
          }
          x21 += 50;
          num137 += 1;
        }
        while (num137 <= 8);
        let mut y27: i32 = num135 + (height2 + 16);
        let mut counter6: i32 = simpleList2.Counter;
        for (let mut index17: i32 = 0; index17 <= counter6; index17 += 1)
        {
          let mut x22: i32 = num136;
          data5: String = self.game.Data.StringListObj[self.slotItemType].GetData(0, simpleList2.Id[index17], 2);
          ttext1: String = simpleStringList.Id[index17];
           let mut local9: &Graphics = &g;
          bitmap1: Bitmap = BitmapStore.GetBitmap(self.game.SE1_SIDEBAR_VARBOX_LONG);
           let mut local10: &Bitmap = &bitmap1;
          let mut x23: i32 = x22;
          let mut y28: i32 = y27;
          DrawMod.DrawSimple( local9,  local10, x23, y28);
          let mut eventPicSlotFor: i32 = self.game.EventRelatedObj.GetEventPicSlotFor(simpleList2.Id[index17], "", "");
           let mut local11: &Graphics = &g;
          bitmap2: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[eventPicSlotFor]);
           let mut local12: &Bitmap = &bitmap2;
          let mut x24: i32 = x22 + 2;
          let mut y29: i32 = y27 + 6;
          DrawMod.DrawSimple( local11,  local12, x24, y29);
          DrawMod.DrawTextColouredConsole( g, data5, self.game.MarcFont16, x22 + 31, y27 + 4, self.game.seColGray);
          rectangle10 = Rectangle::new(x22, y27, 85, height2);
          let mut trect1: &Rectangle = &rectangle10
          self.AddMouse( trect1, data5, ttext1);
          let mut x25: i32 = x22 + 85;
          let mut tdata1: i32 = 1;
          do
          {
            ttitle: String;
            if (!flag1)
            {
              if (tdata1 == 1)
                ttitle = "";
              if (tdata1 == 2)
                ttitle = "SHQ Delivery";
              if (tdata1 == 3)
                ttitle = "";
              if (tdata1 == 4)
                ttitle = "Consumption";
              if (tdata1 == 5)
                ttitle = "Production";
              if (tdata1 == 6)
                ttitle = "";
              if (tdata1 == 7)
                ttitle = "SHQ Pickup";
              if (tdata1 == 8)
                ttitle = "Start turn/Current Zone Reserves";
            }
            else
            {
              if (tdata1 == 1)
                ttitle = "Current SHQ Inventory";
              if (tdata1 == 2)
                ttitle = "Predicted SHQ delivery";
              if (tdata1 == 3)
                ttitle = "Predicted Zone Inventory after SHQ delivery";
              if (tdata1 == 4)
                ttitle = "Predicted Zone Consumption";
              if (tdata1 == 5)
                ttitle = "Predicted Zone Production";
              if (tdata1 == 6)
                ttitle = "Predicted Zone Inventory after Consumption & Production";
              if (tdata1 == 7)
                ttitle = "Predicted SHQ pickup of surplus Items at Zone";
              if (tdata1 == 8)
                ttitle = "Predicted SHQ Inventory at start of next turn";
            }
            bool flag2 = false;
            if (!flag1 & (tdata1 == 2 | tdata1 == 5 | tdata1 == 4 | tdata1 == 7 | tdata1 == 8))
              flag2 = true;
            if (flag1)
              flag2 = true;
            if (flag2)
            {
              str3 = "";
              let mut num138: i32 = 0;
              let mut num139: i32 = 0;
              ttext2: String = "";
              if (flag1)
              {
                let mut nr: i32 = self.PIL.FindNr(simpleList2.Id[index17], tdata1, se1AssetZone, tdata4: 1);
                if (nr > -1)
                {
                  num138 = self.PIL.Weight[nr];
                  num139 = self.PIL.Data3[nr];
                  let mut counter7: i32 = self.PILdesc.Counter;
                  for (let mut index18: i32 = 0; index18 <= counter7; index18 += 1)
                  {
                    if (self.PILdesc.Data1[index18] == tdata1 && self.PILdesc.Data2[index18] == se1AssetZone && self.PILdesc.Data3[index18] == simpleList2.Id[index17])
                      ttext2 = self.PILdesc.Id[index18];
                  }
                }
              }
              else if (!flag1)
              {
                let mut nr: i32 = self.IL.FindNr(simpleList2.Id[index17], tdata1, se1AssetZone, tdata4: 0);
                if (nr > -1)
                {
                  num138 = self.IL.Weight[nr];
                  num139 = self.IL.Data3[nr];
                  let mut counter8: i32 = self.ILdesc.Counter;
                  for (let mut index19: i32 = 0; index19 <= counter8; index19 += 1)
                  {
                    if (self.ILdesc.Data1[index19] == tdata1 && self.ILdesc.Data2[index19] == se1AssetZone && self.ILdesc.Data3[index19] == simpleList2.Id[index17])
                      ttext2 = self.ILdesc.Id[index19];
                  }
                }
              }
              tstring20: String = num138.ToString();
              if (num138 > 9999)
                tstring20 = Math.Round( num138 / 1000.0, 1).ToString() + "k";
              bool flag3 = false;
              if (tdata1 == 4 & self.game.EditObj.se1_assetItemMode1 == simpleList2.Id[index17])
                flag3 = true;
              if (tdata1 == 5 & self.game.EditObj.se1_assetItemMode2 == simpleList2.Id[index17])
                flag3 = true;
              if (flag3)
                DrawMod.DrawBlock( g, x25 - 3, y27 + 4, 46, height2 - 10, 100,  byte.MaxValue, 100, 100);
              c: Color = self.game.seColGray;
              if (num139 > num138 & num139 > 0 & num138 > 0)
                c = self.game.seColYellow;
              else if (num139 > 0 & num138 < 1)
                c = self.game.seColRed;
              else if (num138 == 0)
                tstring20 = "-";
              DrawMod.DrawTextColouredConsole( g, tstring20, self.game.MarcFont16, x25, y27 + 4, c);
              if (tdata1 == 4 | tdata1 == 5)
              {
                if (flag3)
                {
                  ttext3: String = ttext2 + "\r\nYOU HAVE CURRENTLY SELECTED THIS ITEM.\r\nClick again or on other column than Prod+Cons to deselect.";
                  rectangle10 = Rectangle::new(x25, y27, 50, height2);
                  let mut trect2: &Rectangle = &rectangle10
                  self.AddMouse( trect2, ttitle, ttext3, 120 + tdata1, simpleList2.Id[index17]);
                }
                else
                {
                  ttext4: String = ttext2 + "\r\nClick to view only Assets that are concerned.";
                  rectangle10 = Rectangle::new(x25, y27, 50, height2);
                  let mut trect3: &Rectangle = &rectangle10
                  self.AddMouse( trect3, ttitle, ttext4, 120 + tdata1, simpleList2.Id[index17]);
                }
              }
              else if (ttext2.Length > 1)
              {
                rectangle10 = Rectangle::new(x25, y27, 50, height2);
                let mut trect4: &Rectangle = &rectangle10
                self.AddMouse( trect4, ttitle, ttext2, 120);
              }
            }
            x25 += 50;
            tdata1 += 1;
          }
          while (tdata1 <= 8);
          y27 += height2;
        }
      }
      g.Dispose();
      g = (Graphics) null;
    }

    pub HandleKeyup: WindowReturnClass(nr: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          if (nr > 0 & self.okId > 0)
          {
            windowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.okId)] + 1, self.SubPartY[self.SubpartNr(self.okId)] + 1, 1);
            windowReturnClass.SetFlag(true);
          }
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass2: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (self.MouseData[index] > 0 && x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height & b == 1)
        {
          if (self.MouseData[index] == 120)
          {
            self.game.EditObj.se1_assetItemMode2 = 0;
            self.game.EditObj.se1_assetItemMode1 = 0;
            self.dostuff();
            windowReturnClass1.AddCommand(4, 67);
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
          if (self.MouseData[index] == 124)
          {
            self.game.EditObj.se1_assetItemMode2 = 0;
            if (self.game.EditObj.se1_assetItemMode1 == self.MouseData2[index])
              self.game.EditObj.se1_assetItemMode1 = 0;
            else
              self.game.EditObj.se1_assetItemMode1 = self.MouseData2[index];
            self.dostuff();
            windowReturnClass1.AddCommand(4, 67);
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
          if (self.MouseData[index] == 125)
          {
            self.game.EditObj.se1_assetItemMode1 = 0;
            if (self.game.EditObj.se1_assetItemMode2 == self.MouseData2[index])
              self.game.EditObj.se1_assetItemMode2 = 0;
            else
              self.game.EditObj.se1_assetItemMode2 = self.MouseData2[index];
            self.dostuff();
            windowReturnClass1.AddCommand(4, 67);
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
          if (self.MouseData[index] == 121)
          {
            if (self.game.EditObj.se1_SelectAssetButton == self.MouseData2[index])
            {
              self.game.EditObj.UDSpopupText = "";
              self.formref.Cursor = Cursors.WaitCursor;
              self.game.EditObj.UDSClearInput();
              self.game.EventRelatedObj.SetUDSKey("ASSETID", self.game.EditObj.se1_SelectAssetButton);
              self.game.EventRelatedObj.DoCheckSpecificEvent(self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 561, 0, 0));
              self.formref.Cursor = Cursors.Default;
              self.game.EditObj.PopupValue = 21;
              windowReturnClass1.AddCommand(5, 14);
              self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            self.game.EditObj.se1_SelectAssetButton = self.MouseData2[index];
            self.dostuff();
            windowReturnClass1.AddCommand(4, 67);
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
        }
      }
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num1: i32 = self.SubPartID[index1];
            if (num1 == self.RightId)
            {
              if (self.game.EditObj.se1_assetRightPanel == 1)
                self.game.EditObj.se1_assetRightPanel = 0;
              else
                self.game.EditObj.se1_assetRightPanel = 1;
              self.dostuff();
              windowReturnClass1.AddCommand(4, 67);
              windowReturnClass1.SetFlag(true);
              return windowReturnClass1;
            }
            if (num1 == self.PreviewId)
            {
              if (self.game.EditObj.se1_assetMode <= 1)
              {
                self.game.EditObj.se1_assetMode = 2;
                if (!self.previewSet)
                {
                  self.game.ProcessingObj.LIS_SetNetwork(false, true);
                  self.previewSet = true;
                }
                self.dostuff();
                windowReturnClass1.AddCommand(4, 67);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
            }
            else if (num1 == self.StartTurnId)
            {
              if (self.game.EditObj.se1_assetMode > 1)
              {
                self.game.EditObj.se1_assetMode = 1;
                self.dostuff();
                windowReturnClass1.AddCommand(4, 67);
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
            }
            else
            {
              if (num1 == self.listId)
              {
                let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                self.SubPartFlag[index1] = true;
                if (num2 <= -1)
                  return windowReturnClass1;
                self.game.EditObj.se1_assetSHQ = num2;
                self.ReCalculate();
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
              if (num1 == self.list2Id)
              {
                let mut num3: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                self.SubPartFlag[index1] = true;
                if (num3 <= -1)
                  return windowReturnClass1;
                self.game.EditObj.se1_assetZone = num3;
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
            }
            let mut assetButtonCounter: i32 = self.assetButtonCounter;
            for (let mut index2: i32 = 0; index2 <= assetButtonCounter; index2 += 1)
            {
              if (self.assetButton[index2] == self.SubPartID[index1])
              {
                if (self.assetButtonData[index2] >= 51 & self.assetButtonData[index2] < 99)
                {
                  self.game.EditObj.se1_assetPage2 = self.assetButtonData[index2] - 50;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 11)
                {
                  self.game.EditObj.se1_AssetCategory1 = 1;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 12)
                {
                  self.game.EditObj.se1_AssetCategory1 = 2;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 13)
                {
                  if (self.game.EditObj.se1_AssetCategory2 == 1)
                    self.game.EditObj.se1_AssetCategory2 = 0;
                  else
                    self.game.EditObj.se1_AssetCategory2 = 1;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 14)
                {
                  if (self.game.EditObj.se1_AssetCategory2 == 2)
                    self.game.EditObj.se1_AssetCategory2 = 0;
                  else
                    self.game.EditObj.se1_AssetCategory2 = 2;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 15)
                {
                  if (self.game.EditObj.se1_AssetCategory2 == 3)
                    self.game.EditObj.se1_AssetCategory2 = 0;
                  else
                    self.game.EditObj.se1_AssetCategory2 = 3;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] > 160 & self.assetButtonData[index2] < 170)
                {
                  if (self.game.EditObj.se1_AssetCategory5 == self.assetButtonData[index2] - 160)
                    self.game.EditObj.se1_AssetCategory5 = 0;
                  else
                    self.game.EditObj.se1_AssetCategory5 = self.assetButtonData[index2] - 160;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 22)
                {
                  self.game.EditObj.UDSpopupText = "";
                  self.onPopupRefreshReCalc = true;
                  self.formref.Cursor = Cursors.WaitCursor;
                  self.game.EditObj.UDSClearInput();
                  self.game.EventRelatedObj.SetUDSKey("ASSETID", self.game.EditObj.se1_SelectAssetButton);
                  self.game.EventRelatedObj.DoCheckSpecificEvent(self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 569, 0, 0));
                  self.formref.Cursor = Cursors.Default;
                  self.game.EditObj.PopupValue = 21;
                  windowReturnClass1.AddCommand(5, 14);
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 21)
                {
                  self.orderfeedbackString = "";
                  self.AssetOrderNumber = 21;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  if (self.orderfeedbackString.Length > 1)
                  {
                    self.game.EditObj.QuestionText = self.orderfeedbackString;
                    self.game.EditObj.AnswerCount = 1;
                    self.game.EditObj.AnswerText[1] = "OK";
                    self.game.EditObj.AnswerTextMouseOver[1] = "Click to continue";
                    DrawMod.TGame.EditObj.PopupValue = 10;
                    windowReturnClass1.AddCommand(5, 14);
                    self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 23)
                {
                  self.orderfeedbackString = "";
                  self.AssetOrderNumber = 23;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  if (self.orderfeedbackString.Length > 1)
                  {
                    self.game.EditObj.QuestionText = self.orderfeedbackString;
                    self.game.EditObj.AnswerCount = 1;
                    self.game.EditObj.AnswerText[1] = "OK";
                    self.game.EditObj.AnswerTextMouseOver[1] = "Click to continue";
                    DrawMod.TGame.EditObj.PopupValue = 10;
                    windowReturnClass1.AddCommand(5, 14);
                    self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] >= 2000 & self.assetButtonData[index2] <= 2100)
                {
                  self.orderfeedbackString = "";
                  self.AssetOrderNumber = self.assetButtonData[index2];
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 24)
                {
                  self.game.EditObj.UDSpopupText = "";
                  self.formref.Cursor = Cursors.WaitCursor;
                  self.game.EditObj.UDSClearInput();
                  self.game.EventRelatedObj.SetUDSKey("ASSETID", self.game.EditObj.se1_SelectAssetButton);
                  self.game.EventRelatedObj.DoCheckSpecificEvent(self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 561, 0, 0));
                  self.formref.Cursor = Cursors.Default;
                  self.game.EditObj.PopupValue = 21;
                  windowReturnClass1.AddCommand(5, 14);
                  self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 31)
                {
                  self.orderfeedbackString = "";
                  self.AssetOrderNumber = 31;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 32)
                {
                  self.orderfeedbackString = "";
                  self.AssetOrderNumber = 32;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index2] == 33)
                {
                  self.orderfeedbackString = "";
                  self.AssetOrderNumber = 33;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
              }
            }
          }
        }
        windowReturnClass1.SetFlag(false);
        return windowReturnClass1;
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    pub fn PopUpRefresh()
    {
      if (self.onPopupRefreshReCalc)
        self.dostuff();
      self.onPopupRefreshReCalc = false;
    }
  }
}
