// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SpecialWindowClass5
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
  pub class SpecialWindowClass5 : WindowClass
  {
     ListClass ListObj;
     ListClass List2Obj;
     listId: i32;
     list2Id: i32;
     useWidth: i32;
     useHeight: i32;
     SimpleList listShq;
     SimpleList listUnit;
     SimpleList listUnitModel;
     SimpleList listUnitReinf;
     SimpleList listReinf;
     SimpleList listModel;
     SimpleList listModelType;
     SimpleList listUnitTotal;
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
     slotModel: i32;
     slotModelType: i32;
     slotModelTech: i32;
     slotTechType: i32;
     slotModelStatName: i32;
     slotModelStatBefore: i32;
     slotModelStat: i32;
     slotQuality: i32;
     slotChoice: i32;
     int[] itemweight;
     itemName: Vec<String>;
     int[] assetButton;
     assetButtonCounter: i32;
     int[] assetButtonData;
     opt1: i32;
     but0id: i32;
     but1id: i32;
     but2id: i32;
     but3id: i32;
     but4id: i32;
     but5id: i32;

    pub fn Dispose() => base.Dispose();

    pub HandleMouseMove: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      return base.HandleMouseMove(x, y);
    }

    pub SpecialWindowClass5( tGame: GameClass, tUseWidth: i32, tUseHeight: i32)
      : base( tGame, tUseWidth, tUseHeight, 8)
    {
      self.ListObj = ListClass::new();
      self.List2Obj = ListClass::new();
      self.itemweight = new int[100];
      self.itemName = new string[100];
      self.assetButton = new int[600];
      self.assetButtonCounter = -1;
      self.assetButtonData = new int[600];
      self.useWidth = tUseWidth;
      self.useHeight = tUseHeight;
      libName: String = "SE_Data";
      self.slotItemType = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 149, 0, 0));
      self.slotCharacter = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 196, 0, 0));
      self.slotRegKey = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 210, 0, 0));
      self.slotModel = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 228, 0, 0));
      self.slotModelStat = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 363, 0, 0));
      self.slotModelStatName = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 349, 0, 0));
      self.slotModelType = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 225, 0, 0));
      self.slotQuality = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 299, 0, 0));
      self.slotChoice = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 232, 0, 0));
      self.slotModelTech = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 230, 0, 0));
      self.slotTechType = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 190, 0, 0));
      let mut length: i32 = self.game.Data.StringListObj[self.slotItemType].Length;
      for (let mut index1: i32 = 0; index1 <= length; index1 += 1)
      {
        let mut index2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotItemType].Data[index1, 0]));
        let mut num: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotItemType].Data[index1, 3]));
        self.itemweight[index2] = num;
        self.itemName[index2] = self.game.Data.StringListObj[self.slotItemType].Data[index1, 1];
      }
      self.assetButtonCounter = -1;
      self.ReCalculate();
      self.dostuff();
    }

    pub fn ReCalculate()
    {
      self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 381, 0, 0));
      let mut id: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      self.listShq = SimpleList::new();
      self.listUnit = SimpleList::new();
      self.listUnitModel = SimpleList::new();
      self.listUnitReinf = SimpleList::new();
      self.listReinf = SimpleList::new();
      self.listModel = SimpleList::new();
      self.listUnitTotal = SimpleList::new();
      let mut unitCounter1: i32 = self.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter1; index += 1)
      {
        if (self.game.Data.UnitObj[index].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index].PreDef == -1 && self.game.Data.UnitObj[index].Historical > -1 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index].Historical].Type == 8)
        {
          self.listShq.Add(index, 0);
          self.listUnit.Add(index, 0, index);
          if (self.game.EditObj.se1_modelSHQ < 1)
            self.game.EditObj.se1_modelSHQ = index;
        }
      }
      let mut unitCounter2: i32 = self.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter2; index += 1)
      {
        if (self.game.Data.UnitObj[index].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index].PreDef == -1 && self.game.Data.UnitObj[index].Historical > -1 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index].Historical].Type >= 5 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index].Historical].Type < 8)
        {
          let mut topHq: i32 = self.game.HandyFunctionsObj.GetTopHQ(index);
          if (self.listShq.FindNr(topHq) > -1)
            self.listUnit.Add(index, 0, topHq);
        }
      }
      SimpleList simpleList = SimpleList::new();
      let mut unitCounter3: i32 = self.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter3; index += 1)
      {
        if (self.game.Data.UnitObj[index].Regime == self.game.Data.Turn && self.game.Data.UnitObj[index].PreDef == -1 && self.game.Data.UnitObj[index].Historical > -1 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index].Historical].Type < 5 && self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[index].Historical].GiveHisVarValue(11) < 1)
        {
          let mut topHq: i32 = self.game.HandyFunctionsObj.GetTopHQ(index);
          if (self.listShq.FindNr(topHq) > -1)
          {
            let mut hq: i32 = self.game.Data.UnitObj[index].HQ;
            if (hq != topHq)
              simpleList.Add(index, 0, hq);
            else
              self.listUnit.Add(index, 0, topHq);
          }
        }
      }
      let mut counter1: i32 = self.listUnit.Counter;
      ratio: i32;
      qty: i32;
      for (let mut index1: i32 = 0; index1 <= counter1; index1 += 1)
      {
        let mut tid: i32 = self.listUnit.Id[index1];
        let mut sfCount: i32 = self.game.Data.UnitObj[tid].SFCount;
        for (let mut index2: i32 = 0; index2 <= sfCount; index2 += 1)
        {
          let mut sf: i32 = self.game.Data.UnitObj[tid].SFList[index2];
          let mut type: i32 = self.game.Data.SFObj[sf].Type;
          ratio = self.game.Data.SFTypeObj[type].Ratio;
          let mut reinforcementType: i32 = self.game.Data.SFTypeObj[type].ReinforcementType;
          qty = self.game.Data.SFObj[sf].Qty;
          let mut tdata1: i32 = self.game.Data.SFTypeObj[type].SFTypeVar[81];
          if (reinforcementType > -1)
            self.listUnitReinf.AddWeight(tid, ratio * qty, reinforcementType, CheckData1Existence: true);
          if (tdata1 > 0)
            self.listUnitModel.AddWeight(tid, ratio * qty, tdata1, CheckData1Existence: true);
          self.listUnitTotal.AddWeight(tid, ratio * qty);
        }
      }
      let mut counter2: i32 = simpleList.Counter;
      for (let mut index3: i32 = 0; index3 <= counter2; index3 += 1)
      {
        let mut index4: i32 = simpleList.Id[index3];
        let mut tid: i32 = simpleList.Data1[index3];
        let mut sfCount: i32 = self.game.Data.UnitObj[index4].SFCount;
        for (let mut index5: i32 = 0; index5 <= sfCount; index5 += 1)
        {
          let mut sf: i32 = self.game.Data.UnitObj[index4].SFList[index5];
          let mut type: i32 = self.game.Data.SFObj[sf].Type;
          ratio = self.game.Data.SFTypeObj[type].Ratio;
          let mut reinforcementType: i32 = self.game.Data.SFTypeObj[type].ReinforcementType;
          qty = self.game.Data.SFObj[sf].Qty;
          let mut tdata1: i32 = self.game.Data.SFTypeObj[type].SFTypeVar[81];
          if (reinforcementType > -1)
            self.listUnitReinf.AddWeight(tid, ratio * qty, reinforcementType, CheckData1Existence: true);
          if (tdata1 > 0)
            self.listUnitModel.AddWeight(tid, ratio * qty, tdata1, CheckData1Existence: true);
          self.listUnitTotal.AddWeight(tid, ratio * qty);
        }
      }
      let mut length: i32 = self.game.Data.StringListObj[self.slotModel].Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
      {
        if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].Data[index, 2])) == id)
        {
          let mut num1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].Data[index, 5]));
          if (num1 > 0 && self.game.EditObj.se1_modelObsoleteHidden == 0 | Conversions.ToInteger(self.game.Data.StringListObj[self.slotQuality].GetData2(0, num1, 1, id, 2)) != 1)
          {
            let mut tid: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].Data[index, 0]));
            let mut num2: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].Data[index, 1]));
            let mut reinforcementType: i32 = self.game.Data.SFTypeObj[self.game.HandyFunctionsObj.GetSFTypeByID(num1)].ReinforcementType;
            if (reinforcementType > -1)
              self.listReinf.AddWeight(reinforcementType, ratio * qty);
            if (tid > 0)
              self.listModel.AddWeight(tid, ratio * qty, reinforcementType);
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
      SizeF sizeF1 = SizeF::new();
      let mut id: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      libName: String = "SE_Data";
      self.slotItemType = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 149, 0, 0));
      self.slotCharacter = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 196, 0, 0));
      self.slotRegKey = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 210, 0, 0));
      self.slotModel = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 228, 0, 0));
      self.slotModelType = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 225, 0, 0));
      self.slotQuality = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 299, 0, 0));
      self.slotChoice = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 232, 0, 0));
      self.slotModelStat = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 363, 0, 0));
      self.slotModelStatBefore = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 229, 0, 0));
      self.slotModelTech = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 230, 0, 0));
      self.slotTechType = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 190, 0, 0));
      let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 169, 0, 0));
      self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 168, 0, 0));
      let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID(libName, 143, 0, 0));
      let mut stringListById3: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 306, 0, 0));
      let mut turn: i32 = self.game.Data.Turn;
      let mut idValue1: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, id, 2)));
      let mut cultureGroupId: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById3].GetData(0, idValue1, 1)));
      self.game.Data.StringListObj[stringListById1].SetData(0, "REGIMEID", 1, id);
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
      let mut tTop: i32 = -1;
      if (self.list2Id > 0)
      {
        tTop = self.SubPartList[self.SubpartNr(self.list2Id)].GetTopItem();
        self.RemoveSubPart(self.list2Id);
        self.list2Id = 0;
      }
      if (self.but0id > 0)
      {
        self.RemoveSubPart(self.but0id);
        self.but0id = 0;
      }
      if (self.but1id > 0)
      {
        self.RemoveSubPart(self.but1id);
        self.but1id = 0;
      }
      if (self.but2id > 0)
      {
        self.RemoveSubPart(self.but2id);
        self.but2id = 0;
      }
      if (self.but3id > 0)
      {
        self.RemoveSubPart(self.but3id);
        self.but3id = 0;
      }
      if (self.but4id > 0)
      {
        self.RemoveSubPart(self.but4id);
        self.but4id = 0;
      }
      if (self.but5id > 0)
      {
        self.RemoveSubPart(self.but5id);
        self.but5id = 0;
      }
      if (self.opt1 > 0)
      {
        self.RemoveSubPart(self.opt1);
        self.opt1 = 0;
      }
      let mut assetButtonCounter1: i32 = self.assetButtonCounter;
      for (let mut index: i32 = 0; index <= assetButtonCounter1; index += 1)
      {
        if (self.assetButton[index] > 0)
        {
          self.RemoveSubPart(self.assetButton[index]);
          self.assetButton[index] = 0;
          self.assetButtonData[index] = 0;
        }
      }
      self.assetButtonCounter = -1;
      let mut y1: i32 = 80;
      let mut height1: i32 =  Math.Round( (self.useHeight - (100 + y1)) / 2.0);
      Rectangle rectangle1 = Rectangle::new(0, y1, 220, self.useHeight);
      let mut width1: i32 = rectangle1.Width;
      let mut y2: i32 = y1;
      let mut width2: i32 = self.useWidth - width1;
      let mut height2: i32 = 50;
      Rectangle rectangle2 = Rectangle::new(width1, y2, width2, height2);
      Rectangle rectangle3 = Rectangle::new(rectangle2.Left, rectangle2.Top + rectangle2.Height, rectangle2.Width, height1);
      Rectangle rectangle4 = Rectangle::new(rectangle3.X, rectangle3.Y + rectangle3.Height, rectangle3.Width, 50);
      Rectangle rectangle5 = Rectangle::new(rectangle4.Left, rectangle4.Top + rectangle4.Height, rectangle4.Width, height1);
      Rectangle rectangle6 = Rectangle::new(10, rectangle3.Top + 10, 210, rectangle3.Height - 30);
      Rectangle rectangle7 = Rectangle::new(10, rectangle5.Top + 10, 210, rectangle5.Height - 30);
      DrawMod.DrawBlock( g, rectangle1.X, rectangle1.Y, rectangle1.Width, rectangle1.Height, 0, 0, 0, 120);
      DrawMod.DrawBlock( g, rectangle2.X, rectangle2.Y, rectangle2.Width, rectangle2.Height, 0, 0, 0, 160);
      DrawMod.DrawBlock( g, rectangle4.X, rectangle4.Y, rectangle4.Width, rectangle4.Height, 0, 0, 0, 160);
      DrawMod.DrawBlock( g, rectangle3.X, rectangle3.Y, rectangle3.Width, rectangle3.Height, 0, 0, 0, 60);
      DrawMod.DrawBlock( g, rectangle5.X, rectangle5.Y, rectangle5.Width, rectangle5.Height, 0, 0, 0, 60);
      let mut left1: i32 = rectangle6.Left;
      let mut num1: i32 = rectangle6.Top - 40;
      let mut tsubpart1: SubPartClass =  new MarcRadioPartClass(0, self.game.EditObj.se1_modelObsoleteHidden == 1, "Click to show or hide Obsolete Quality Level Models",  self.BackBitmap, left1, num1);
      self.opt1 = self.AddSubPart( tsubpart1, left1, num1, 35, 35, 1);
      DrawMod.DrawTextColouredConsole( g, "Hide Obsolete", self.game.MarcFont4, left1 + 40, num1 + 8, self.game.seColWhite);
      self.ListObj = ListClass::new();
      let mut left2: i32 = rectangle7.Left;
      let mut num2: i32 = rectangle7.Top + 10;
      let mut twidth: i32 = rectangle7.Width - 10;
      let mut tlistsize1: i32 =  Math.Round(Math.Floor( rectangle7.Height / 20.0)) - 1;
      let mut tlistselect1: i32 = -1;
      let mut num3: i32 = 1 - 1;
      self.ListObj.add("All", -2);
      if (self.game.EditObj.se1_modelSHQ < 1)
        tlistselect1 = num3;
      let mut counter1: i32 = self.listShq.Counter;
      for (let mut index: i32 = 0; index <= counter1; index += 1)
      {
        num3 += 1;
        if (self.listShq.Id[index] == self.game.EditObj.se1_modelSHQ)
          tlistselect1 = num3;
        self.ListObj.add(self.game.Data.UnitObj[self.listShq.Id[index]].Name, self.listShq.Id[index]);
      }
      let mut tsubpart2: SubPartClass =  new ListSubPartClass(self.ListObj, tlistsize1, twidth, tlistselect1, self.game, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: left2, bby: num2, tMarcStyle: true, overruleFont: ( self.game.MarcFont7), overruleItemSize: 20);
      self.listId = self.AddSubPart( tsubpart2, left2, num2, twidth + 20, (tlistsize1 + 1) * 20, 1);
      self.List2Obj = ListClass::new();
      let mut left3: i32 = rectangle6.Left;
      let mut num4: i32 = rectangle6.Top + 10;
      let mut num5: i32 = rectangle6.Width - 10;
      let mut tlistsize2: i32 =  Math.Round(Math.Floor( rectangle6.Height / 20.0)) - 1;
      let mut tlistselect2: i32 = -1;
      let mut num6: i32 = 1 - 1;
      self.List2Obj.add("All", -2);
      if (self.game.EditObj.se1_modelReinf < 0)
        tlistselect2 = num6;
      let mut counter2: i32 = self.listReinf.Counter;
      for (let mut index: i32 = 0; index <= counter2; index += 1)
      {
        num6 += 1;
        if (self.listReinf.Id[index] == self.game.EditObj.se1_modelReinf)
          tlistselect2 = num6;
        self.List2Obj.add(self.game.Data.ReinfName[self.listReinf.Id[index]], self.listReinf.Id[index]);
      }
      if (tTop < 1)
        tTop = 0;
      if (tTop == 0)
        tTop = -1;
      tsubpart2 =  new ListSubPartClass(self.List2Obj, tlistsize2, num5, tlistselect2, self.game, tTop: tTop, tShowPair: true, tValueWidth: 70, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: left3, bby: num4, tMarcStyle: true, overruleFont: ( self.game.MarcFont7), overruleItemSize: 20);
      self.list2Id = self.AddSubPart( tsubpart2, left3, num4, num5, (tlistsize2 + 1) * 20, 1);
      let mut x1: i32 = rectangle2.Left + 10;
      let mut y3: i32 = rectangle2.Top + 10;
      str1: String = self.game.EditObj.se1_modelReinf <= -1 ? "No Reinforcement Type selected" : "Models for Reinforcement Type '" + self.game.Data.ReinfName[self.game.EditObj.se1_modelReinf] + "'";
      DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont2, x1, y3, Color.White);
      SizeF sizeF2 = g.MeasureString(str1, self.game.MarcFont2);
      let mut x2: i32 =  Math.Round( rectangle2.Left +  sizeF2.Width + 40.0);
      let mut num7: i32 = 75;
      let mut y4: i32 = rectangle2.Top + 3;
      tsubpart2 =  new SEBigTextPartClass("Quality", "Change data viewed for Models", self.game.EditObj.se1_modelView == 0, num7, 44);
      self.but0id = self.AddSubPart( tsubpart2, x2, y4, num7, 44, 1);
      let mut x3: i32 = x2 + 72;
      tsubpart2 =  new SEBigTextPartClass("Prd.Cost", "Change data viewed for Models", self.game.EditObj.se1_modelView == 1, num7, 44);
      self.but1id = self.AddSubPart( tsubpart2, x3, y4, num7, 44, 1);
      let mut x4: i32 = x3 + 72;
      tsubpart2 =  new SEBigTextPartClass("Op.Cost", "Change data viewed for Models", self.game.EditObj.se1_modelView == 2, num7, 44);
      self.but2id = self.AddSubPart( tsubpart2, x4, y4, num7, 44, 1);
      let mut x5: i32 = x4 + 72;
      tsubpart2 =  new SEBigTextPartClass("Design", "Change data viewed for Models", self.game.EditObj.se1_modelView == 3, num7, 44);
      self.but3id = self.AddSubPart( tsubpart2, x5, y4, num7, 44, 1);
      if (self.game.EventRelatedObj.Helper_AirEnabled())
      {
        x5 += 72;
        tsubpart2 =  new SEBigTextPartClass("Air.Des", "Change data viewed for Models", self.game.EditObj.se1_modelView == 5, num7, 44);
        self.but5id = self.AddSubPart( tsubpart2, x5, y4, num7, 44, 1);
      }
      let mut x6: i32 = x5 + 72;
      tsubpart2 =  new SEBigTextPartClass("Techs", "Change data viewed for Models", self.game.EditObj.se1_modelView == 4, num7, 44);
      self.but4id = self.AddSubPart( tsubpart2, x6, y4, num7, 44, 1);
      let mut left4: i32 = rectangle3.Left;
      let mut top1: i32 = rectangle3.Top;
      let mut num8: i32 = 40 +  Math.Round( Math.Max(0, self.useHeight - 800) / 25.0);
      if (num8 > 80)
        num8 = 80;
      let mut width3: i32 = rectangle3.Width;
      let mut num9: i32 =  Math.Round(Math.Floor( (rectangle3.Height - 20) /  num8));
      let mut num10: i32 = 0;
      let mut counter3: i32 = self.listModel.Counter;
      for (let mut index: i32 = 0; index <= counter3; index += 1)
      {
        if (self.listModel.Data1[index] == self.game.EditObj.se1_modelReinf | self.game.EditObj.se1_modelReinf < 0)
          num10 += 1;
      }
      let mut num11: i32 = 1 +  Math.Round(Math.Floor( (num10 - 1) /  num9));
      if (num11 < self.game.EditObj.se1_modelPage)
        self.game.EditObj.se1_modelPage = num11;
      if (self.game.EditObj.se1_modelPage < 1)
        self.game.EditObj.se1_modelPage = 1;
      let mut num12: i32 = (self.game.EditObj.se1_modelPage - 1) * num9 + 1;
      let mut num13: i32 =  Math.Round(Math.Floor( (rectangle2.Width - (x6 + 76 - rectangle2.Left)) /  num11));
      if (num13 > 100)
        num13 = 100;
      let mut num14: i32 = num13 - 4;
      let mut num15: i32 = left4;
      let mut num16: i32 = top1;
      let mut x7: i32 = rectangle2.Right - (num14 + 4) * num11;
      let mut y5: i32 = rectangle2.Top + 3;
      let mut num17: i32 = num11;
      for (let mut index: i32 = 1; index <= num17; index += 1)
      {
        this += 1.assetButtonCounter;
        str1 = index.ToString() + "/" + num11.ToString() + ". Click to view this Models page.";
        if (self.game.EditObj.se1_modelPage == index)
          str1 = index.ToString() + "/" + num11.ToString() + ". Currently selected Models page";
        int[] assetButton = self.assetButton;
        let mut assetButtonCounter2: i32 = self.assetButtonCounter;
        tsubpart2 =  new SEBigTextPartClass(index.ToString(), str1, self.game.EditObj.se1_modelPage == index, num14, 44);
        let mut num18: i32 = self.AddSubPart( tsubpart2, x7, y5, num14, 44, 1);
        assetButton[assetButtonCounter2] = num18;
        self.assetButtonData[self.assetButtonCounter] = 50 + index;
        x7 += num14 + 4;
      }
      let mut x1_1: i32 = num15;
      let mut y1_1: i32 = num16;
      DrawMod.DrawBlock( g, x1_1, y1_1, width3 - 10, 19, 168, 168, 168, 70);
      let mut num19: i32 = y1_1 + 20;
      let mut num20: i32 = 0;
      let mut num21: i32 = 0;
      let mut num22: i32 = num19;
      let mut counter4: i32 = self.listModel.Counter;
      num23: i32;
      num24: i32;
      tdata1: i32;
      Rectangle trect1;
      Rectangle rectangle8;
      index1: i32;
      for (let mut index2: i32 = 0; index2 <= counter4; index2 += 1)
      {
        if (self.listModel.Data1[index2] == self.game.EditObj.se1_modelReinf | self.game.EditObj.se1_modelReinf < 0)
        {
          num21 += 1;
          if (num21 >= num12 & num21 <= num12 - 1 + num9)
          {
            num20 += 1;
            let mut left5: i32 = rectangle3.Left;
            if (self.listModel.Id[index2] != self.game.EditObj.se1_modelSelected)
              DrawMod.DrawBlock( g, left5, num19, width3 - 10, num8 - 1, 168, 168, 168, 140);
            else
              DrawMod.DrawBlock( g, left5, num19, width3 - 10, num8 - 1, 148, 218, 148, 140);
            let mut idValue2: i32 = self.listModel.Id[index2];
            let mut x8: i32 = left5 + 10;
            let mut sfTypeById: i32 = self.game.HandyFunctionsObj.GetSFTypeByID( Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, idValue2, 5))));
            bitmap: Bitmap = self.game.CustomBitmapObj.DrawSFTypeGraphic(sfTypeById, false, cultureGroupId, self.game.Data.Turn, -1);
            if (bitmap.Width > 70)
            {
              num23 =  Math.Round( (bitmap.Height * 70) /  bitmap.Width);
              num24 = 70;
              tdata1 =  Math.Round( (num8 - num23) / 2.0);
              if (num23 > num8)
              {
                num24 =  Math.Round( (num24 * num8) /  num23);
                num23 = num8;
                tdata1 = 0;
              }
            }
            else if (bitmap.Height > num8)
            {
              num23 = num8;
              num24 =  Math.Round( (bitmap.Width * num8) /  bitmap.Height);
              tdata1 = 0;
            }
            else
            {
              num23 = bitmap.Height;
              num24 = bitmap.Width;
              tdata1 =  Math.Round( (num8 - bitmap.Height) / 2.0);
            }
            if (num20 == 1)
              DrawMod.DrawTextColouredMarc( g, "ICON", self.game.MarcFont5, x8, num19 - 16, Color.White);
             let mut local1: &Graphics = &g;
             let mut local2: &Bitmap = &bitmap;
            trect1 = Rectangle::new(0, 0, bitmap.Width, bitmap.Height);
            let mut srcrect: &Rectangle = &trect1
            rectangle8 = Rectangle::new(x8 - 6, num19 + tdata1, num24, num23);
            let mut destrect: &Rectangle = &rectangle8
            DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
            bitmap.Dispose();
            bitmap = (Bitmap) null;
            rectangle8 = Rectangle::new(x8, num19, 70, num8);
            let mut trect2: &Rectangle = &rectangle8
            self.AddMouse( trect2, "", "Click to get more information on this Model", 20000000 + self.listModel.Id[index2]);
            let mut x9: i32 = x8 + 70;
            if (self.game.EditObj.se1_modelView == 0)
            {
              rectangle8 = Rectangle::new(x9, num19, 290, num8);
              let mut trect3: &Rectangle = &rectangle8
              self.AddMouse( trect3, "", "Click to select this row", 1000000 + self.listModel.Id[index2]);
              rectangle8 = Rectangle::new(x9 + 360 + 360, num19, width3 - 720, num8);
              trect1 = rectangle8;
              self.AddMouse( trect1, "", "Click to select this row", 1000000 + self.listModel.Id[index2]);
            }
            else
            {
              rectangle8 = Rectangle::new(x9, num19, width3 - 360, num8);
              trect1 = rectangle8;
              self.AddMouse( trect1, "", "Click to select this row", 1000000 + self.listModel.Id[index2]);
            }
            data1: String = self.game.Data.StringListObj[self.slotModel].GetData(0, idValue2, 3);
            SizeF sizeF3 = g.MeasureString(data1, self.game.MarcFont4);
            let mut num25: i32 =  Math.Round(( num8 -  Math.Max(15f, sizeF3.Height)) / 2.0);
            if (num20 == 1)
              DrawMod.DrawTextColouredMarc( g, "MODEL", self.game.MarcFont5, x9, num19 - 16, Color.White);
            DrawMod.DrawTextColouredMarc( g, data1, self.game.MarcFont16, x9, num19 + num25 - 10, Color.White);
            let mut idValue3: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, idValue2, 1)));
            let mut idValue4: i32 = idValue3;
            data2: String = self.game.Data.StringListObj[self.slotModelType].GetData(0, idValue3, 1);
            DrawMod.DrawTextColouredMarc( g, data2, self.game.MarcFont4, x9, num19 + num25 + 10, Color.White);
            let mut x10: i32 = x9 + 160;
            if (self.game.EditObj.se1_modelView <= 2)
            {
              num24 = self.game.HandyFunctionsObj.GetSFTypeCombatValueScore(sfTypeById);
              tstring: String = num24.ToString();
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "COMBAT", self.game.MarcFont5, x10, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont4, x10, num19 + num25, Color.White);
              let mut x11: i32 = x10 + 60;
              num24 = self.game.HandyFunctionsObj.GetSFTypeProdCostScore(sfTypeById);
              str1 = num24.ToString();
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "COST", self.game.MarcFont5, x11, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, x11, num19 + num25, Color.White);
              x10 = x11 + 60;
            }
            if (self.game.EditObj.se1_modelView == 0)
            {
              let mut integer: i32 = Conversions.ToInteger(self.game.Data.StringListObj[self.slotQuality].GetData2(0, self.game.Data.SFTypeObj[sfTypeById].Id, 1, id, 2));
              let mut num26: i32 = 0;
              do
              {
                bool flag = false;
                if (integer == num26)
                  flag = true;
                str2: String = "";
                if (num26 == 0)
                {
                  str1 = "None";
                  str2 = str1;
                }
                if (num26 == 1)
                {
                  str1 = "Obsl.";
                  str2 = "Obsolete";
                }
                if (num26 == 2)
                {
                  str1 = "Low";
                  str2 = str1;
                }
                if (num26 == 3)
                {
                  str1 = "Reg.";
                  str2 = "Regular";
                }
                if (num26 == 4)
                {
                  str1 = "High";
                  str2 = str1;
                }
                if (num26 == 5)
                {
                  str1 = "Elite";
                  str2 = str1;
                }
                upper: String = str2.ToUpper();
                if (num26 == 1 | num26 == 3 | num26 == 5)
                  DrawMod.DrawBlock( g, x10 - 5, num19, 60, num8 - 1, 0, 0, 0, 40);
                else
                  DrawMod.DrawBlock( g, x10 - 5, num19, 60, num8 - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 40);
                if (num20 == 1)
                  DrawMod.DrawTextColouredMarc( g, str1.ToUpper(), self.game.MarcFont5, x10, num19 - 16, Color.White);
                if (flag)
                {
                  DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont16, x10, num19 + num25, Color.White);
                  rectangle8 = Rectangle::new(x10, num19, 60, num8);
                  trect1 = rectangle8;
                  self.AddMouse( trect1, "Current Quality Level: " + upper, "This is the current Quality Level assigned to this Model.");
                }
                else
                {
                  rectangle8 = Rectangle::new(x10, num19, 60, num8);
                  trect1 = rectangle8;
                  self.AddMouse( trect1, "Change to Quality Level: " + upper, "Click to change the Model to this Quality Level.", 10000000 + num26 * 1000000 + self.listModel.Id[index2]);
                }
                x10 += 60;
                num26 += 1;
              }
              while (num26 <= 5);
              let mut idValue5: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelType].GetData(0, idValue4, 4)));
              let mut idValue6: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelType].GetData(0, idValue4, 5)));
              let mut idValue7: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelType].GetData(0, idValue4, 15)));
              let mut idValue8: i32 = 0;
              let mut idValue9: i32 = 0;
              let mut idValue10: i32 = 0;
              let mut idValue11: i32 = 0;
              if (self.game.EventRelatedObj.Helper_AirEnabled())
              {
                idValue8 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelType].GetData(0, idValue4, 21)));
                idValue9 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelType].GetData(0, idValue4, 22)));
                idValue10 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelType].GetData(0, idValue4, 23)));
                idValue11 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelType].GetData(0, idValue4, 24)));
              }
              str1 = "";
              let mut num27: i32 = 0;
              if (idValue5 > 0)
              {
                num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotChoice].GetData(0, idValue5, 1)));
                if (num24 > 0 & !(num24 == 531 | num24 == 533 | num24 == 535))
                {
                  num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, num24, 2)));
                  data2_1: String = self.game.Data.StringListObj[self.slotChoice].GetData2(1, num24, 2, num23, 3);
                  if (Operators.CompareString(data2_1, "None", false) != 0 & Strings.InStr(data2_1, "No ") < 1)
                  {
                    if (data2_1.Length > 0 & str1.Length > 0)
                    {
                      if (((Strings.InStr(str1, ",") > 0 ? 1 : 0) & 0) != 0)
                      {
                        str1 += ",\r\n";
                        num27 = 0;
                      }
                      else
                      {
                        str1 += ", ";
                        num27 = 1;
                      }
                    }
                    if (data2_1.Length > 0)
                      str1 += data2_1;
                  }
                }
              }
              if (idValue6 > 0)
              {
                num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotChoice].GetData(0, idValue6, 1)));
                if (num24 > 0 & !(num24 == 531 | num24 == 533 | num24 == 535))
                {
                  num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, num24, 2)));
                  data2_2: String = self.game.Data.StringListObj[self.slotChoice].GetData2(1, num24, 2, num23, 3);
                  if (Operators.CompareString(data2_2, "None", false) != 0 & Strings.InStr(data2_2, "No ") < 1)
                  {
                    if (data2_2.Length > 0 & str1.Length > 0)
                    {
                      if (((Strings.InStr(str1, ",") > 0 ? 1 : 0) & 0) != 0)
                      {
                        str1 += ",\r\n";
                        num27 = 0;
                      }
                      else
                      {
                        str1 += ", ";
                        num27 = 1;
                      }
                    }
                    if (data2_2.Length > 0)
                      str1 += data2_2;
                  }
                }
              }
              if (idValue7 > 0)
              {
                num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotChoice].GetData(0, idValue7, 1)));
                if (num24 > 0 & !(num24 == 531 | num24 == 533 | num24 == 535))
                {
                  num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, num24, 2)));
                  data2_3: String = self.game.Data.StringListObj[self.slotChoice].GetData2(1, num24, 2, num23, 3);
                  if (Operators.CompareString(data2_3, "None", false) != 0 & Strings.InStr(data2_3, "No ") < 1)
                  {
                    if (data2_3.Length > 0 & str1.Length > 0)
                    {
                      if (((Strings.InStr(str1, ",") > 0 ? 1 : 0) & 0) != 0)
                      {
                        str1 += ",\r\n";
                        num27 = 0;
                      }
                      else
                      {
                        str1 += ", ";
                        num27 = 1;
                      }
                    }
                    if (data2_3.Length > 0)
                      str1 += data2_3;
                  }
                }
              }
              if (idValue8 > 0)
              {
                num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotChoice].GetData(0, idValue8, 1)));
                if (num24 > 0 & !(num24 == 531 | num24 == 533 | num24 == 535))
                {
                  num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, num24, 2)));
                  data2_4: String = self.game.Data.StringListObj[self.slotChoice].GetData2(1, num24, 2, num23, 3);
                  if (Operators.CompareString(data2_4, "None", false) != 0 & Strings.InStr(data2_4, "No ") < 1)
                  {
                    if (data2_4.Length > 0 & str1.Length > 0)
                    {
                      if (((Strings.InStr(str1, ",") > 0 ? 1 : 0) & 0) != 0)
                      {
                        if (num27 > 0)
                        {
                          str1 += ",\r\n";
                          num27 = 0;
                        }
                        else
                        {
                          str1 += ", ";
                          num27 = 1;
                        }
                      }
                      else
                      {
                        str1 += ", ";
                        num27 = 1;
                      }
                    }
                    if (data2_4.Length > 0)
                      str1 += data2_4;
                  }
                }
              }
              if (idValue9 > 0)
              {
                num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotChoice].GetData(0, idValue9, 1)));
                if (num24 > 0 & !(num24 == 531 | num24 == 533 | num24 == 535))
                {
                  num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, num24, 2)));
                  data2_5: String = self.game.Data.StringListObj[self.slotChoice].GetData2(1, num24, 2, num23, 3);
                  if (Operators.CompareString(data2_5, "None", false) != 0 & Strings.InStr(data2_5, "No ") < 1)
                  {
                    if (data2_5.Length > 0 & str1.Length > 0)
                    {
                      if (((Strings.InStr(str1, ",") > 0 ? 1 : 0) & 0) != 0)
                      {
                        if (num27 > 0)
                        {
                          str1 += ",\r\n";
                          num27 = 0;
                        }
                        else
                        {
                          str1 += ", ";
                          num27 = 1;
                        }
                      }
                      else
                      {
                        str1 += ", ";
                        num27 = 1;
                      }
                    }
                    if (data2_5.Length > 0)
                      str1 += data2_5;
                  }
                }
              }
              if (idValue10 > 0)
              {
                num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotChoice].GetData(0, idValue10, 1)));
                if (num24 > 0 & !(num24 == 531 | num24 == 533 | num24 == 535))
                {
                  num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, num24, 2)));
                  data2_6: String = self.game.Data.StringListObj[self.slotChoice].GetData2(1, num24, 2, num23, 3);
                  if (Operators.CompareString(data2_6, "None", false) != 0 & Strings.InStr(data2_6, "No ") < 1)
                  {
                    if (data2_6.Length > 0 & str1.Length > 0)
                    {
                      if (((Strings.InStr(str1, ",") > 0 ? 1 : 0) & 0) != 0)
                      {
                        if (num27 > 0)
                        {
                          str1 += ",\r\n";
                          num27 = 0;
                        }
                        else
                        {
                          str1 += ", ";
                          num27 = 1;
                        }
                      }
                      else
                      {
                        str1 += ", ";
                        num27 = 1;
                      }
                    }
                    if (data2_6.Length > 0)
                      str1 += data2_6;
                  }
                }
              }
              if (idValue11 > 0)
              {
                num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotChoice].GetData(0, idValue11, 1)));
                if (num24 > 0 & !(num24 == 531 | num24 == 533 | num24 == 535))
                {
                  num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, num24, 2)));
                  data2_7: String = self.game.Data.StringListObj[self.slotChoice].GetData2(1, num24, 2, num23, 3);
                  if (Operators.CompareString(data2_7, "None", false) != 0 & Strings.InStr(data2_7, "No ") < 1)
                  {
                    num28: i32;
                    if (data2_7.Length > 0 & str1.Length > 0)
                    {
                      if (((Strings.InStr(str1, ",") > 0 ? 1 : 0) & 0) != 0)
                      {
                        if (num27 > 0)
                        {
                          str1 += ",\r\n";
                          num28 = 0;
                        }
                        else
                        {
                          str1 += ", ";
                          num28 = 1;
                        }
                      }
                      else
                      {
                        str1 += ", ";
                        num28 = 1;
                      }
                    }
                    if (data2_7.Length > 0)
                      str1 += data2_7;
                  }
                }
              }
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "VARIANTS", self.game.MarcFont5, x10, num19 - 16, Color.White);
              sizeF1 = g.MeasureString(str1, self.game.MarcFont4);
              if (str1.Length > 110)
              {
                if (Strings.InStr(str1, "\r\n") > 0)
                  DrawMod.DrawTextColouredConsoleMultiline( g, str1, self.game.marcFont17, x10, num19 + num25 - 17, Color.White, 300, num8);
                else
                  DrawMod.DrawTextColouredConsoleMultiline( g, str1, self.game.marcFont17, x10, num19 + num25 - 17, Color.White, 300, num8);
              }
              else if (Strings.InStr(str1, "\r\n") > 0)
                DrawMod.DrawTextColouredConsoleMultiline( g, str1, self.game.MarcFont4, x10, num19 + num25 - 17, Color.White, 300, num8);
              else
                DrawMod.DrawTextColouredConsoleMultiline( g, str1, self.game.MarcFont4, x10, num19 + num25 - 17, Color.White, 300, num8);
              x10 += 320;
              if (x10 + 80 < self.game.ScreenWidth - 20)
              {
                num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStatBefore].GetData2(0, idValue2, 1, 7, 2)));
                num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStatBefore].GetData2(0, idValue2, 1, 8, 2)));
                tdata1 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, idValue2, 4)));
                let mut num29: i32 = 120 + tdata1 * 10;
                let mut num30: i32 = 100 -  Math.Round( (100 * (num29 - num24)) /  (num29 - num23));
                if (num30 > 100)
                  num30 = 100;
                str1 = num30.ToString() + "%";
                if (num20 == 1)
                  DrawMod.DrawTextColouredMarc( g, "FIELD TEST", self.game.MarcFont5, x10, num19 - 16, Color.White);
                DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, x10, num19 + num25, Color.White);
                x10 += 80;
              }
            }
            if (self.game.EditObj.se1_modelView == 1)
            {
              let mut num31: i32 = 1;
              do
              {
                num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 400 + num31, 2)));
                DrawMod.DrawBlock( g, x10 - 5, num19, 100, num8 - 1, 0, 0, 0, 40);
                tstring: String = "";
                if (num31 == 1)
                {
                  tstring = "Manpower";
                  num23 *= 100;
                }
                if (num31 == 2)
                  tstring = "Metal";
                if (num31 == 3)
                  tstring = "IP";
                if (num31 == 4)
                  tstring = "Machinery";
                if (num31 == 5)
                  tstring = "Hi-Tech Parts";
                if (num31 == 6)
                  tstring = "Radioactives";
                if (num31 == 7)
                  tstring = "Rare metals";
                str1 = num23.ToString();
                if (num23 <= 0)
                  str1 = "-";
                if (num20 == 1)
                  DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont5, x10, num19 - 16, Color.White);
                DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, x10, num19 + num25, Color.White);
                x10 += 100;
                num31 += 1;
              }
              while (num31 <= 7);
            }
            double num32;
            if (self.game.EditObj.se1_modelView == 2)
            {
              let mut num33: i32 = 1;
              do
              {
                let mut idValue2_1: i32 = 0;
                if (num33 == 1 | num33 == 2 | num33 == 3)
                  DrawMod.DrawBlock( g, x10 - 5, num19, 100, num8 - 1, 0, 0, 0, 40);
                else if (num33 == 4 | num33 == 5 | num33 == 6)
                  DrawMod.DrawBlock( g, x10 - 5, num19, 100, num8 - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 40);
                tstring: String = "";
                if (num33 == 1)
                {
                  tstring = "Move Oil";
                  idValue2_1 = 101;
                }
                if (num33 == 2)
                {
                  tstring = "Move Energy";
                  idValue2_1 = 102;
                }
                if (num33 == 3)
                {
                  tstring = "Move Radio.";
                  idValue2_1 = 103;
                }
                if (num33 == 4)
                {
                  tstring = "Combat Ammo";
                  idValue2_1 = 201;
                }
                if (num33 == 5)
                {
                  tstring = "Combat Energy";
                  idValue2_1 = 202;
                }
                if (num33 == 6)
                {
                  tstring = "Combat Radio.";
                  idValue2_1 = 203;
                }
                if (num33 == 7)
                {
                  tstring = "Upkeep Food";
                  idValue2_1 = 301;
                }
                if (num33 == 8)
                {
                  tstring = "Upkeep Energy";
                  idValue2_1 = 302;
                }
                num23 = 0;
                if (num33 == 1)
                {
                  num24 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[45];
                  if (num24 == 1)
                    num23 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[46];
                }
                else if (num33 == 2)
                {
                  num24 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[45];
                  if (num24 == 15)
                    num23 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[46];
                }
                else if (num33 == 3)
                {
                  num24 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[45];
                  if (num24 == 4)
                    num23 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[46];
                }
                else
                  num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, idValue2_1, 2)));
                str1 = num23.ToString();
                if (num33 == 7 && self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[50] == 7 & self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[68] > 1)
                {
                  num32 = Math.Round( ( self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[51] /  self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[68]), 1);
                  str1 = num32.ToString();
                }
                if (num23 <= 0)
                  str1 = "-";
                if (num33 == 4 & num23 > 0 && self.game.Data.SFTypeObj[sfTypeById].Theater == 2)
                {
                  tdata1 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 529, 2)));
                  str1 = str1 + "/" + tdata1.ToString();
                }
                if (num20 == 1)
                  DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont5, x10, num19 - 16, Color.White);
                DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, x10, num19 + num25, Color.White);
                x10 += 100;
                num33 += 1;
              }
              while (num33 <= 8);
            }
            if (self.game.EditObj.se1_modelView == 5 && self.game.EventRelatedObj.Helper_AirEnabled())
            {
              DrawMod.DrawBlock( g, x10 - 5, num19, 80, num8 - 1, 0, 0, 0, 40);
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 504, 2)));
              tstring1: String = num23.ToString();
              if (self.game.Data.SFTypeObj[sfTypeById].Theater < 2)
                tstring1 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "ENGINE.EFF", self.game.MarcFont5, x10, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring1, self.game.MarcFont4, x10, num19 + num25, Color.White);
              let mut x12: i32 = x10 + 80;
              DrawMod.DrawBlock( g, x12 - 5, num19, 80, num8 - 1, 0, 0, 0, 40);
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 510, 2)));
              tstring2: String = num23.ToString() + "%";
              if (self.game.Data.SFTypeObj[sfTypeById].Theater < 2)
                tstring2 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "HP:WGT RATIO", self.game.MarcFont5, x12, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring2, self.game.MarcFont4, x12, num19 + num25, Color.White);
              let mut x13: i32 = x12 + 80;
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 525, 2)));
              tstring3: String = num23.ToString();
              if (num23 <= 0)
                tstring3 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "FIREP.vs.AIR", self.game.MarcFont5, x13, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring3, self.game.MarcFont4, x13, num19 + num25, Color.White);
              let mut x14: i32 = x13 + 80;
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 536, 2)));
              tdata1 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 517, 2)));
              tstring4: String = num23.ToString() + "kmh / \r\n" + tdata1.ToString() + "kmh";
              if (num23 <= 0 | tdata1 <= 0)
                tstring4 = "-";
              if (self.game.Data.SFTypeObj[sfTypeById].Theater < 2)
                tstring4 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "TAKEOFF", self.game.MarcFont5, x14, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring4, self.game.MarcFont4, x14, num19 + num25 - 8, Color.White);
              let mut x15: i32 = x14 + 80;
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 516, 2)));
              tstring5: String = num23.ToString() + "kmh";
              if (num23 <= 0)
                tstring5 = "-";
              if (self.game.Data.SFTypeObj[sfTypeById].Theater < 2)
                tstring5 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "MAX SPEED", self.game.MarcFont5, x15, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring5, self.game.MarcFont4, x15, num19 + num25, Color.White);
              let mut x16: i32 = x15 + 80;
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 509, 2)));
              tstring6: String = num23 < 50 ? "No, Aero:" + num23.ToString() : "Yes, Aero:" + num23.ToString();
              if (num23 <= 0)
                tstring6 = "-";
              if (self.game.Data.SFTypeObj[sfTypeById].Theater < 2)
                tstring6 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "HYPERSONIC", self.game.MarcFont5, x16, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring6, self.game.MarcFont4, x16, num19 + num25, Color.White);
              let mut x17: i32 = x16 + 100;
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 520, 2)));
              tstring7: String = num23.ToString();
              if (num23 <= 0)
                tstring7 = "-";
              if (self.game.Data.SFTypeObj[sfTypeById].Theater < 2)
                tstring7 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "OP.RANGE", self.game.MarcFont5, x17, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring7, self.game.MarcFont4, x17, num19 + num25, Color.White);
              let mut x18: i32 = x17 + 60;
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 502, 2)));
              tstring8: String = num23.ToString() + " kg";
              if (num23 >= 1000)
              {
                num32 = Math.Round( num23 / 1000.0, 1);
                tstring8 = num32.ToString() + " ton";
              }
              if (num23 <= 0)
                tstring8 = "-";
              if (self.game.Data.SFTypeObj[sfTypeById].Theater < 2)
                tstring8 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "FUEL.CAP", self.game.MarcFont5, x18, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring8, self.game.MarcFont4, x18, num19 + num25, Color.White);
              let mut x19: i32 = x18 + 80;
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 503, 2)));
              tstring9: String = num23.ToString() + "kg";
              if (num23 >= 1000)
              {
                num32 = Math.Round( num23 / 1000.0, 1);
                tstring9 = num32.ToString() + " ton";
              }
              if (num23 <= 0)
                tstring9 = "-";
              if (self.game.Data.SFTypeObj[sfTypeById].Theater < 2)
                tstring9 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "CARGO.CAP", self.game.MarcFont5, x19, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring9, self.game.MarcFont4, x19, num19 + num25, Color.White);
              let mut x20: i32 = x19 + 80;
              DrawMod.DrawBlock( g, x20 - 5, num19, 80, num8 - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 40);
              num24 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[14] * self.game.Data.SFTypeObj[sfTypeById].Attacks;
              tstring10: String = num24.ToString();
              if (num24 <= 0)
                tstring10 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "AIR ATT LOW", self.game.MarcFont5, x20, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring10, self.game.MarcFont4, x20, num19 + num25, Color.White);
              let mut x21: i32 = x20 + 80;
              DrawMod.DrawBlock( g, x21 - 5, num19, 80, num8 - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 40);
              num24 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[21] * self.game.Data.SFTypeObj[sfTypeById].Attacks;
              tstring11: String = num24.ToString();
              if (num24 <= 0)
                tstring11 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "AIR ATT HIGH", self.game.MarcFont5, x21, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring11, self.game.MarcFont4, x21, num19 + num25, Color.White);
              let mut x22: i32 = x21 + 80;
              DrawMod.DrawBlock( g, x22 - 5, num19, 80, num8 - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 40);
              num24 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[13];
              tstring12: String = num24.ToString();
              if (num24 <= 0)
                tstring12 = "-";
              if (num24 <= 0)
                tstring12 = "-";
              if (self.game.Data.SFTypeObj[sfTypeById].Theater < 2)
                tstring12 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "DOGFIGHT", self.game.MarcFont5, x22, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring12, self.game.MarcFont4, x22, num19 + num25, Color.White);
              let mut x23: i32 = x22 + 80;
              DrawMod.DrawBlock( g, x23 - 5, num19, 80, num8 - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 40);
              num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 530, 2)));
              str1 = self.game.HandyFunctionsObj.GetRomanNumerical(num24);
              if (self.game.Data.SFTypeObj[sfTypeById].Theater < 2)
                str1 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "Min.Airbase", self.game.MarcFont5, x23, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, x23, num19 + num25, Color.White);
              x10 = x23 + 80;
            }
            if (self.game.EditObj.se1_modelView == 3)
            {
              DrawMod.DrawBlock( g, x10 - 5, num19, 80, num8 - 1, 0, 0, 0, 40);
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 151, 2)));
              tstring13: String = num23.ToString();
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "STR.DESIGN", self.game.MarcFont5, x10, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring13, self.game.MarcFont4, x10, num19 + num25, Color.White);
              let mut x24: i32 = x10 + 80;
              DrawMod.DrawBlock( g, x24 - 5, num19, 80, num8 - 1, 0, 0, 0, 40);
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStatBefore].GetData2(0, idValue2, 1, 8, 2)));
              tstring14: String = num23.ToString();
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "B.DESIGN", self.game.MarcFont5, x24, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring14, self.game.MarcFont4, x24, num19 + num25, Color.White);
              let mut x25: i32 = x24 + 80;
              DrawMod.DrawBlock( g, x25 - 5, num19, 80, num8 - 1, 0, 0, 0, 40);
              num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 43, 2)));
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 25, 2)));
              tstring15: String = num23.ToString();
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "ENG.DESIGN", self.game.MarcFont5, x25, num19 - 16, Color.White);
              if (num24 < 1)
                tstring15 = "-";
              DrawMod.DrawTextColouredMarc( g, tstring15, self.game.MarcFont4, x25, num19 + num25, Color.White);
              let mut x26: i32 = x25 + 80;
              DrawMod.DrawBlock( g, x26 - 5, num19, 80, num8 - 1, 0, 0, 0, 40);
              num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 41, 2)));
              tdata1 = 0;
              if (self.game.EventRelatedObj.Helper_AirEnabled())
                tdata1 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 534, 2)));
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 27, 2)));
              tstring16: String = num23.ToString();
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "WEAP.DESIGN", self.game.MarcFont5, x26, num19 - 16, Color.White);
              if (num24 < 1 & tdata1 < 1)
                tstring16 = "-";
              DrawMod.DrawTextColouredMarc( g, tstring16, self.game.MarcFont4, x26, num19 + num25, Color.White);
              let mut x27: i32 = x26 + 80;
              DrawMod.DrawBlock( g, x27 - 5, num19, 80, num8 - 1, 0, 0, 0, 40);
              num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 42, 2)));
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 26, 2)));
              tstring17: String = num23.ToString();
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "ARM.DESIGN", self.game.MarcFont5, x27, num19 - 16, Color.White);
              if (num24 < 1 & self.game.Data.SFTypeObj[sfTypeById].Theater < 2)
                tstring17 = "-";
              DrawMod.DrawTextColouredMarc( g, tstring17, self.game.MarcFont4, x27, num19 + num25, Color.White);
              let mut x28: i32 = x27 + 80;
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 2, 2)));
              tstring18: String = num23.ToString();
              if (num23 <= 0)
                tstring18 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "FIRE POWER", self.game.MarcFont5, x28, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring18, self.game.MarcFont4, x28, num19 + num25, Color.White);
              let mut x29: i32 = x28 + 80;
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 5, 2)));
              tstring19: String = num23.ToString();
              if (num23 <= 0)
                tstring19 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "WEIGHT", self.game.MarcFont5, x29, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring19, self.game.MarcFont4, x29, num19 + num25, Color.White);
              let mut x30: i32 = x29 + 80;
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 3, 2)));
              tstring20: String = num23.ToString();
              if (num23 <= 0)
                tstring20 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "ENG.POWER", self.game.MarcFont5, x30, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, tstring20, self.game.MarcFont4, x30, num19 + num25, Color.White);
              let mut x31: i32 = x30 + 80;
              num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStat].GetData2(0, idValue2, 1, 4, 2)));
              str1 = num23.ToString();
              if (num23 <= 0)
                str1 = "-";
              if (num20 == 1)
                DrawMod.DrawTextColouredMarc( g, "ARMOUR.STR.", self.game.MarcFont5, x31, num19 - 16, Color.White);
              DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, x31, num19 + num25, Color.White);
              x10 = x31 + 80;
              if (x10 + 80 < x10 + width3)
              {
                num24 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStatBefore].GetData2(0, idValue2, 1, 7, 2)));
                num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelStatBefore].GetData2(0, idValue2, 1, 8, 2)));
                tdata1 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, idValue2, 4)));
                let mut num34: i32 = 120 + tdata1 * 10;
                let mut num35: i32 = 100 -  Math.Round( (100 * (num34 - num24)) /  (num34 - num23));
                if (num35 > 100)
                  num35 = 100;
                str1 = num35.ToString() + "%";
                if (num20 == 1)
                  DrawMod.DrawTextColouredMarc( g, "FIELD TEST", self.game.MarcFont5, x10, num19 - 16, Color.White);
                DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, x10, num19 + num25, Color.White);
                x10 += 80;
              }
              if (x10 + 100 < self.game.ScreenWidth - 20)
              {
                DrawMod.DrawBlock( g, x10 - 5, num19, 100, num8 - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 40);
                num23 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[40];
                num24 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[34];
                str1 = "i" + num23.ToString() + " / " + num24.ToString();
                if (num23 <= 0 & num24 <= 0)
                  str1 = "-";
                if (num20 == 1)
                  DrawMod.DrawTextColouredMarc( g, "HIT POINTS", self.game.MarcFont5, x10, num19 - 16, Color.White);
                DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, x10, num19 + num25, Color.White);
                x10 += 100;
              }
              num36: i32;
              if (x10 + 80 < self.game.ScreenWidth - 20)
              {
                DrawMod.DrawBlock( g, x10 - 5, num19, 80, num8 - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 40);
                num24 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[30] * self.game.Data.SFTypeObj[sfTypeById].Attacks;
                num23 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[31] * self.game.Data.SFTypeObj[sfTypeById].Attacks;
                str1 = num24.ToString() + "/" + num23.ToString();
                if (num24 > 9999 | num23 > 9999)
                {
                  index1 =  Math.Round( num24 / 1000.0);
                  str3: String = index1.ToString();
                  num36 =  Math.Round( num23 / 1000.0);
                  str4: String = num36.ToString();
                  str1 = str3 + "k/" + str4 + "k";
                }
                if (num23 <= 0 & num24 <= 0)
                  str1 = "-";
                if (num20 == 1)
                  DrawMod.DrawTextColouredMarc( g, "SOFT ATT", self.game.MarcFont5, x10, num19 - 16, Color.White);
                DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, x10, num19 + num25, Color.White);
                x10 += 80;
              }
              if (x10 + 80 < self.game.ScreenWidth - 20)
              {
                DrawMod.DrawBlock( g, x10 - 5, num19, 80, num8 - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 40);
                num24 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[32] * self.game.Data.SFTypeObj[sfTypeById].Attacks;
                num23 = self.game.Data.SFTypeObj[sfTypeById].SFTypeVar[33] * self.game.Data.SFTypeObj[sfTypeById].Attacks;
                str1 = num24.ToString() + "/" + num23.ToString();
                if (num24 > 9999 | num23 > 9999)
                {
                  num36 =  Math.Round( num24 / 1000.0);
                  str5: String = num36.ToString();
                  index1 =  Math.Round( num23 / 1000.0);
                  str6: String = index1.ToString();
                  str1 = str5 + "k/" + str6 + "k";
                }
                if (num23 <= 0 & num24 <= 0)
                  str1 = "-";
                if (num20 == 1)
                  DrawMod.DrawTextColouredMarc( g, "HARD ATT", self.game.MarcFont5, x10, num19 - 16, Color.White);
                DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, x10, num19 + num25, Color.White);
                x10 += 80;
              }
            }
            if (self.game.EditObj.se1_modelView == 4 & self.game.Data.StringListObj[self.slotModelTech].Width >= 4)
            {
              bool[] flagArray = new bool[16];
              let mut index3: i32 = 1;
              do
              {
                let mut num37: i32 = 0;
                let mut idValue12: i32 = -1;
                let mut num38: i32 = -1;
                let mut idValue13: i32 = -1;
                let mut num39: i32 = -1;
                let mut length: i32 = self.game.Data.StringListObj[self.slotModelTech].Length;
                for (tdata1 = 0; tdata1 <= length; tdata1 += 1)
                {
                  if ( Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelTech].Data[tdata1, 0])) == idValue2)
                  {
                    num37 += 1;
                    if (num37 == index3)
                    {
                      idValue12 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelTech].Data[tdata1, 1]));
                      num38 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelTech].Data[tdata1, 2]));
                      idValue13 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelTech].Data[tdata1, 3]));
                      num39 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModelTech].Data[tdata1, 4]));
                      break;
                    }
                  }
                }
                if (idValue12 > -1 & num38 > -1 & idValue13 > 0 & num39 > -1)
                {
                  tdata1 =  Math.Round( (self.game.ScreenWidth - 500) / 9.0);
                  if (x10 + tdata1 < self.game.ScreenWidth - 20)
                  {
                    let mut Length: i32 =  Math.Round( tdata1 / 10.0);
                    str7: String = self.game.Data.StringListObj[self.slotTechType].GetData(0, idValue12, 1);
                    if (str7.Length > Length)
                      str7 = Strings.Left(str7, Length);
                    let mut num40: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotTechType].GetData(0, idValue12, 2)));
                    str8: String = self.game.Data.StringListObj[self.slotModelStatName].GetData(0, idValue13, 1);
                    if (str8.Length > Length)
                      str8 = Strings.Left(str8, Length);
                    if (str7.Length > 0 & str8.Length > 0)
                    {
                      str9: String = "";
                      switch (num40)
                      {
                        case 1:
                          str9 = str7;
                          break;
                        case 2:
                          str9 = str7 + " [" + num38.ToString() + "]";
                          break;
                      }
                      str1 = str9 + "\r\n" + str8;
                      if (num39 > 0)
                        str1 = str1 + " +" + num39.ToString();
                      if (-(flagArray[index3] ? 1 : 0) == 0)
                      {
                        DrawMod.DrawTextColouredMarc( g, "TECH BONUS #" + index3.ToString(), self.game.MarcFont5, x10, num22 - 16, Color.White);
                        flagArray[index3] = true;
                      }
                      if (num38 < 1 | num39 < 1)
                        DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont4, x10, num19 +  Math.Round( num25 / 2.0), Color.LightGray);
                      else
                        DrawMod.DrawTextColouredMarc( g, str1, self.game.MarcFont16, x10, num19 +  Math.Round( num25 / 2.0), Color.White);
                      x10 += tdata1;
                    }
                  }
                }
                index3 += 1;
              }
              while (index3 <= 15);
            }
            num19 += num8;
          }
        }
      }
      let mut x32: i32 = rectangle4.Left + 10;
      let mut y6: i32 = rectangle4.Top + 10;
      bool flag1 = false;
      bool flag2 = false;
      str10: String;
      if (self.game.EditObj.se1_modelSelected > 0)
      {
        str10 = "Units TOE for selected Model '" + self.game.Data.StringListObj[self.slotModel].GetData(0, self.game.EditObj.se1_modelSelected, 3) + "'";
        flag2 = true;
      }
      else if (self.game.EditObj.se1_modelReinf > -1)
      {
        str10 = "Units TOE for selected Reinforcement Type '" + self.game.Data.ReinfName[self.game.EditObj.se1_modelReinf] + "'";
        flag1 = true;
      }
      else
      {
        str10 = "Units TOE for all Troops";
        flag1 = true;
      }
      DrawMod.DrawTextColouredMarc( g, str10, self.game.MarcFont2, x32, y6, Color.White);
      let mut left6: i32 = rectangle5.Left;
      let mut top2: i32 = rectangle5.Top;
      let mut height3: i32 = 20 +  Math.Round( Math.Max(0, self.useHeight - 800) / 35.0);
      if (height3 > 40)
        height3 = 40;
      let mut width4: i32 = rectangle5.Width;
      let mut num41: i32 =  Math.Round(Math.Floor( (rectangle5.Height - 20) /  height3));
      let mut num42: i32 = 0;
      let mut counter5: i32 = self.listUnit.Counter;
      for (let mut index4: i32 = 0; index4 <= counter5; index4 += 1)
      {
        if (self.listUnit.Data1[index4] == self.game.EditObj.se1_modelSHQ | self.game.EditObj.se1_modelSHQ <= 0)
          num42 += 1;
      }
      num11 = 1 +  Math.Round(Math.Floor( (num42 - 1) /  num41));
      if (num11 < self.game.EditObj.se1_modelPage2)
        self.game.EditObj.se1_modelPage2 = num11;
      if (self.game.EditObj.se1_modelPage2 < 1)
        self.game.EditObj.se1_modelPage2 = 1;
      let mut num43: i32 = (self.game.EditObj.se1_modelPage2 - 1) * num41 + 1;
      num24 =  Math.Round(Math.Floor( (rectangle2.Width - 500) /  num11));
      if (num24 > 100)
        num24 = 100;
      let mut num44: i32 = left6;
      let mut num45: i32 = top2;
      let mut x33: i32 = rectangle4.Right - (10 + (num24 + 4) * num11);
      let mut y7: i32 = rectangle4.Top + 3;
      let mut num46: i32 = num11;
      for (let mut index5: i32 = 1; index5 <= num46; index5 += 1)
      {
        this += 1.assetButtonCounter;
        str10 = index5.ToString() + "/" + num11.ToString() + ". Click to view this Units page.";
        if (self.game.EditObj.se1_modelPage2 == index5)
          str10 = index5.ToString() + "/" + num11.ToString() + ". Currently selected Units page";
        int[] assetButton = self.assetButton;
        let mut assetButtonCounter3: i32 = self.assetButtonCounter;
        tsubpart2 =  new SEBigTextPartClass(index5.ToString(), str10, self.game.EditObj.se1_modelPage2 == index5, num24, 44);
        let mut num47: i32 = self.AddSubPart( tsubpart2, x33, y7, num24, 44, 1);
        assetButton[assetButtonCounter3] = num47;
        self.assetButtonData[self.assetButtonCounter] = 100 + index5;
        x33 += num24 + 4;
      }
      let mut x1_2: i32 = num44;
      let mut y1_2: i32 = num45;
      DrawMod.DrawBlock( g, x1_2, y1_2, width4 - 10, 19, 168, 168, 168, 70);
      let mut num48: i32 = y1_2 + 20;
      let mut num49: i32 = 0;
      let mut num50: i32 = 0;
      let mut counter6: i32 = self.listUnit.Counter;
      for (let mut index6: i32 = 0; index6 <= counter6; index6 += 1)
      {
        if (self.listUnit.Data1[index6] == self.game.EditObj.se1_modelSHQ | self.game.EditObj.se1_modelSHQ < 1)
        {
          num50 += 1;
          if (num50 >= num43 & num50 <= num43 - 1 + num41)
          {
            num49 += 1;
            let mut left7: i32 = rectangle5.Left;
            let mut tid: i32 = self.listUnit.Id[index6];
            let mut historical: i32 = self.game.Data.UnitObj[tid].Historical;
            if (self.game.Data.HistoricalUnitObj[historical].Type == 8)
              DrawMod.DrawBlock( g, left7, num48, width4 - 10, height3 - 1, 168, 168, 168, 140);
            else if (self.game.Data.HistoricalUnitObj[historical].Type >= 5)
              DrawMod.DrawBlock( g, left7, num48, width4 - 10, height3 - 1, 138, 138, 138, 120);
            else
              DrawMod.DrawBlock( g, left7, num48, width4 - 10, height3 - 1, 108, 108, 108, 120);
            let mut x34: i32 = left7 + 10;
            name: String = self.game.Data.UnitObj[tid].Name;
            SizeF sizeF4 = g.MeasureString(name, self.game.MarcFont4);
            let mut num51: i32 =  Math.Round(( height3 -  Math.Max(15f, sizeF4.Height)) / 2.0);
            if (num49 == 1)
              DrawMod.DrawTextColouredMarc( g, "UNIT NAME", self.game.MarcFont5, x34, num48 - 16, Color.White);
            DrawMod.DrawTextColouredMarc( g, name, self.game.MarcFont4, x34, num48 + num51, Color.White);
            let mut x35: i32 = x34 + 400;
            tstring21: String = Conversions.ToString(self.listUnitTotal.FindWeight(tid));
            if (num49 == 1)
              DrawMod.DrawTextColouredMarc( g, "TOE.TOT", self.game.MarcFont5, x35, num48 - 16, Color.White);
            DrawMod.DrawTextColouredMarc( g, tstring21, self.game.MarcFont4, x35, num48 + num51, Color.White);
            let mut x36: i32 = x35 + 60;
            tstring22: String;
            if (flag1)
            {
              tstring22 = self.game.EditObj.se1_modelReinf <= -1 ? Conversions.ToString(self.listUnitTotal.FindWeight(tid)) : Conversions.ToString(self.listUnitReinf.FindWeight(tid, self.game.EditObj.se1_modelReinf));
              if (num49 == 1)
                DrawMod.DrawTextColouredMarc( g, "TOE.REINF", self.game.MarcFont5, x36, num48 - 16, Color.White);
            }
            else if (flag2)
            {
              tstring22 = Conversions.ToString(self.listUnitModel.FindWeight(tid, self.game.EditObj.se1_modelSelected));
              if (num49 == 1)
                DrawMod.DrawTextColouredMarc( g, "TOE.MODEL", self.game.MarcFont5, x36, num48 - 16, Color.White);
            }
            else
              tstring22 = "";
            DrawMod.DrawTextColouredMarc( g, tstring22, self.game.MarcFont4, x36, num48 + num51, Color.White);
            let mut x37: i32 = x36 + 80;
            int[] numArray1 = new int[7];
            let mut index7: i32 = 0;
            do
            {
              numArray1[index7] = 0;
              index7 += 1;
            }
            while (index7 <= 6);
            let mut counter7: i32 = self.listUnitModel.Counter;
            for (let mut index8: i32 = 0; index8 <= counter7; index8 += 1)
            {
              if (self.listUnitModel.Id[index8] == tid)
              {
                num24 = self.listUnitModel.Data1[index8];
                if (flag2 & (num24 == self.game.EditObj.se1_modelSelected | self.game.EditObj.se1_modelSelected < 1))
                {
                  num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, num24, 5)));
                  tdata1 = Conversions.ToInteger(self.game.Data.StringListObj[self.slotQuality].GetData2(0, num23, 1, id, 2));
                  int[] numArray2 = numArray1;
                  int[] numArray3 = numArray2;
                  index1 = tdata1;
                  let mut index9: i32 = index1;
                  let mut num52: i32 = numArray2[index1] + self.listUnitModel.Weight[index8];
                  numArray3[index9] = num52;
                }
                else if (flag1)
                {
                  num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, num24, 5)));
                  if (self.game.Data.SFTypeObj[self.game.HandyFunctionsObj.GetSFTypeByID(num23)].ReinforcementType == self.game.EditObj.se1_modelReinf | self.game.EditObj.se1_modelReinf == -1)
                  {
                    tdata1 = Conversions.ToInteger(self.game.Data.StringListObj[self.slotQuality].GetData2(0, num23, 1, id, 2));
                    int[] numArray4 = numArray1;
                    int[] numArray5 = numArray4;
                    index1 = tdata1;
                    let mut index10: i32 = index1;
                    let mut num53: i32 = numArray4[index1] + self.listUnitModel.Weight[index8];
                    numArray5[index10] = num53;
                  }
                }
              }
            }
            let mut index11: i32 = 0;
            do
            {
              str11: String = "";
              if (index11 == 0)
              {
                str10 = "None";
                str11 = str10;
              }
              if (index11 == 1)
              {
                str10 = "Obsl.";
                str11 = "Obsolete";
              }
              if (index11 == 2)
              {
                str10 = "Low";
                str11 = str10;
              }
              if (index11 == 3)
              {
                str10 = "Reg.";
                str11 = "Regular";
              }
              if (index11 == 4)
              {
                str10 = "High";
                str11 = str10;
              }
              if (index11 == 5)
              {
                str10 = "Elite";
                str11 = str10;
              }
              let mut x38: i32 = x37;
              upper: String = str11.ToUpper();
              if (index11 == 1 | index11 == 3 | index11 == 5)
                DrawMod.DrawBlock( g, x37 - 5, num48, 80, height3 - 1, 0, 0, 0, 40);
              else
                DrawMod.DrawBlock( g, x37 - 5, num48, 80, height3 - 1,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 40);
              let mut num54: i32 = 0;
              if (index11 >= 2)
                num54 = self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[tid].Historical].GiveHisVarValue(40 + index11);
              tstring23: String = "";
              str12: String = "";
              if (num54 == 0)
              {
                str12 = "Allow";
                tstring23 = "All.";
              }
              if (num54 == 1)
              {
                str12 = "Tolerate";
                tstring23 = "Tol.";
              }
              if (num54 == 10)
              {
                str12 = "Disallow";
                tstring23 = "Dis.";
              }
              if (num49 == 1)
                DrawMod.DrawTextColouredMarc( g, str10.ToUpper(), self.game.MarcFont5, x37, num48 - 16, Color.White);
              if (index11 < 2)
                tstring23 = "";
              if (self.game.Data.HistoricalUnitObj[historical].Type == 8)
                tstring23 = "";
              str13: String = numArray1[index11].ToString();
              SizeF sizeF5 = g.MeasureString(str13, self.game.MarcFont4);
              if (numArray1[index11] > 0)
                DrawMod.DrawTextColouredMarc( g, str13, self.game.MarcFont4, x38, num48 + num51, Color.White);
              else
                DrawMod.DrawTextColouredMarc( g, str13, self.game.MarcFont4, x38, num48 + num51, Color.Gray);
              let mut x39: i32 =  Math.Round( ( x38 + sizeF5.Width));
              if (num54 == 0)
                DrawMod.DrawTextColouredMarc( g, tstring23, self.game.MarcFont16, x39, num48 + num51, Color.Green);
              if (num54 == 1)
                DrawMod.DrawTextColouredMarc( g, tstring23, self.game.MarcFont16, x39, num48 + num51, Color.Yellow);
              if (num54 == 10)
                DrawMod.DrawTextColouredMarc( g, tstring23, self.game.MarcFont16, x39, num48 + num51, Color.OrangeRed);
              if (tstring23.Length > 0)
              {
                rectangle8 = Rectangle::new(x37, num48, 80, height3);
                trect1 = rectangle8;
                self.AddMouse( trect1, "Quality level '" + upper + "' is " + str12.ToUpper(), "Click to change setting", 20000000 + 1000000 * index11 + tid);
              }
              x37 += 80;
              index11 += 1;
            }
            while (index11 <= 5);
            let mut num55: i32 = rectangle5.Right - 10 - x37;
            if (num55 >= 100)
            {
              if (num49 == 1)
                DrawMod.DrawTextColouredMarc( g, "PROMINENT MODELS", self.game.MarcFont5, x37, num48 - 16, Color.White);
              SimpleList simpleList = SimpleList::new();
              let mut counter8: i32 = self.listUnitModel.Counter;
              for (let mut index12: i32 = 0; index12 <= counter8; index12 += 1)
              {
                if (self.listUnitModel.Id[index12] == tid)
                {
                  num24 = self.listUnitModel.Data1[index12];
                  if (flag2 & (num24 == self.game.EditObj.se1_modelSelected | self.game.EditObj.se1_modelSelected < 1))
                  {
                    num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, num24, 5)));
                    tdata1 = Conversions.ToInteger(self.game.Data.StringListObj[self.slotQuality].GetData2(0, num23, 1, id, 2));
                    simpleList.AddWeight(num23, self.listUnitModel.Weight[index12], tdata1);
                  }
                  else if (flag1)
                  {
                    num23 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, num24, 5)));
                    if (self.game.Data.SFTypeObj[self.game.HandyFunctionsObj.GetSFTypeByID(num23)].ReinforcementType == self.game.EditObj.se1_modelReinf | self.game.EditObj.se1_modelReinf == -1)
                    {
                      tdata1 = Conversions.ToInteger(self.game.Data.StringListObj[self.slotQuality].GetData2(0, num23, 1, id, 2));
                      simpleList.AddWeight(num23, self.listUnitModel.Weight[index12], tdata1);
                    }
                  }
                }
              }
              simpleList.ReverseSortHighSpeed();
              let mut counter9: i32 = simpleList.Counter;
              for (let mut index13: i32 = 0; index13 <= counter9; index13 += 1)
              {
                let mut num56: i32 = simpleList.Weight[index13];
                let mut sfTypeById: i32 = self.game.HandyFunctionsObj.GetSFTypeByID(simpleList.Id[index13]);
                bitmap: Bitmap = self.game.CustomBitmapObj.DrawSFTypeGraphic(sfTypeById, false, cultureGroupId, self.game.Data.Turn, -1);
                if (bitmap.Width > 50)
                {
                  num23 =  Math.Round( (bitmap.Height * 50) /  bitmap.Width);
                  num24 = 50;
                  tdata1 =  Math.Round( (height3 - num23) / 2.0);
                  if (num23 > height3)
                  {
                    num24 =  Math.Round( (num24 * height3) /  num23);
                    num23 = height3;
                    tdata1 = 0;
                  }
                }
                else if (bitmap.Height > height3)
                {
                  num23 = height3;
                  num24 =  Math.Round( (bitmap.Width * height3) /  bitmap.Height);
                  tdata1 = 0;
                }
                else
                {
                  num23 = bitmap.Height;
                  num24 = bitmap.Width;
                  tdata1 =  Math.Round( (height3 - bitmap.Height) / 2.0);
                }
                 let mut local3: &Graphics = &g;
                 let mut local4: &Bitmap = &bitmap;
                rectangle8 = Rectangle::new(0, 0, bitmap.Width, bitmap.Height);
                let mut srcrect: &Rectangle = &rectangle8
                trect1 = Rectangle::new(x37 - 6, num48 + tdata1, num24, num23);
                let mut destrect: &Rectangle = &trect1
                DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
                bitmap.Dispose();
                bitmap = (Bitmap) null;
                let mut x40: i32 = x37;
                let mut x41: i32 = x37 + (num24 - 10);
                str14: String = "x" + num56.ToString();
                SizeF sizeF6 = g.MeasureString(str14, self.game.MarcFont4);
                DrawMod.DrawTextColouredMarc( g, str14, self.game.MarcFont4, x41, num48 + num51, Color.White);
                x37 =  Math.Round( ( x41 + (sizeF6.Width + 5f)));
                num55 =  Math.Round( ( num55 -  ( num24 +  sizeF6.Width + 5.0)));
                rectangle8 = Rectangle::new(x40, num48, x37 - x40, height3);
                trect1 = rectangle8;
                self.AddMouse( trect1, self.game.Data.SFTypeObj[sfTypeById].Name, "There are " + num56.ToString() + " of this model in this Unit.");
                if (num55 < 100)
                  break;
              }
            }
            num48 += height3;
          }
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
      let mut id: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].id;
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass2: WindowReturnClass = WindowReturnClass::new();
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index1: i32 = 0; index1 <= mouseCounter; index1 += 1)
      {
        if (self.MouseData[index1] > 0 && x > self.MouseRect[index1].X & x < self.MouseRect[index1].X + self.MouseRect[index1].Width && y > self.MouseRect[index1].Y & y < self.MouseRect[index1].Y + self.MouseRect[index1].Height & b == 1)
        {
          if (self.MouseData[index1] > 22000000 & self.MouseData[index1] < 23000000)
          {
            let mut index2: i32 = self.MouseData[index1] - 22000000;
            let mut historical1: i32 = self.game.Data.UnitObj[index2].Historical;
            let mut num: i32 = self.game.Data.HistoricalUnitObj[historical1].GiveHisVarValue(42);
            switch (num)
            {
              case 0:
                num = 1;
                break;
              case 1:
                num = 10;
                break;
              case 10:
                num = 0;
                break;
            }
            self.game.Data.HistoricalUnitObj[historical1].SetHisVarValue(42, num);
            if (self.game.Data.HistoricalUnitObj[historical1].Type == 5)
            {
              let mut unitCounter: i32 = self.game.Data.UnitCounter;
              for (let mut index3: i32 = 0; index3 <= unitCounter; index3 += 1)
              {
                if (self.game.Data.UnitObj[index3].HQ == index2)
                {
                  let mut historical2: i32 = self.game.Data.UnitObj[index3].Historical;
                  if (historical2 > -1)
                    self.game.Data.HistoricalUnitObj[historical2].SetHisVarValue(42, num);
                }
              }
            }
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 23000000 & self.MouseData[index1] < 24000000)
          {
            let mut index4: i32 = self.MouseData[index1] - 23000000;
            let mut historical3: i32 = self.game.Data.UnitObj[index4].Historical;
            let mut num: i32 = self.game.Data.HistoricalUnitObj[historical3].GiveHisVarValue(43);
            switch (num)
            {
              case 0:
                num = 1;
                break;
              case 1:
                num = 10;
                break;
              case 10:
                num = 0;
                break;
            }
            self.game.Data.HistoricalUnitObj[historical3].SetHisVarValue(43, num);
            if (self.game.Data.HistoricalUnitObj[historical3].Type == 5)
            {
              let mut unitCounter: i32 = self.game.Data.UnitCounter;
              for (let mut index5: i32 = 0; index5 <= unitCounter; index5 += 1)
              {
                if (self.game.Data.UnitObj[index5].HQ == index4)
                {
                  let mut historical4: i32 = self.game.Data.UnitObj[index5].Historical;
                  if (historical4 > -1)
                    self.game.Data.HistoricalUnitObj[historical4].SetHisVarValue(43, num);
                }
              }
            }
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 24000000 & self.MouseData[index1] < 25000000)
          {
            let mut index6: i32 = self.MouseData[index1] - 24000000;
            let mut historical5: i32 = self.game.Data.UnitObj[index6].Historical;
            let mut num: i32 = self.game.Data.HistoricalUnitObj[historical5].GiveHisVarValue(44);
            switch (num)
            {
              case 0:
                num = 1;
                break;
              case 1:
                num = 10;
                break;
              case 10:
                num = 0;
                break;
            }
            self.game.Data.HistoricalUnitObj[historical5].SetHisVarValue(44, num);
            if (self.game.Data.HistoricalUnitObj[historical5].Type == 5)
            {
              let mut unitCounter: i32 = self.game.Data.UnitCounter;
              for (let mut index7: i32 = 0; index7 <= unitCounter; index7 += 1)
              {
                if (self.game.Data.UnitObj[index7].HQ == index6)
                {
                  let mut historical6: i32 = self.game.Data.UnitObj[index7].Historical;
                  if (historical6 > -1)
                    self.game.Data.HistoricalUnitObj[historical6].SetHisVarValue(44, num);
                }
              }
            }
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 25000000 & self.MouseData[index1] < 26000000)
          {
            let mut index8: i32 = self.MouseData[index1] - 25000000;
            let mut historical7: i32 = self.game.Data.UnitObj[index8].Historical;
            let mut num: i32 = self.game.Data.HistoricalUnitObj[historical7].GiveHisVarValue(45);
            switch (num)
            {
              case 0:
                num = 1;
                break;
              case 1:
                num = 10;
                break;
              case 10:
                num = 0;
                break;
            }
            self.game.Data.HistoricalUnitObj[historical7].SetHisVarValue(45, num);
            if (self.game.Data.HistoricalUnitObj[historical7].Type == 5)
            {
              let mut unitCounter: i32 = self.game.Data.UnitCounter;
              for (let mut index9: i32 = 0; index9 <= unitCounter; index9 += 1)
              {
                if (self.game.Data.UnitObj[index9].HQ == index8)
                {
                  let mut historical8: i32 = self.game.Data.UnitObj[index9].Historical;
                  if (historical8 > -1)
                    self.game.Data.HistoricalUnitObj[historical8].SetHisVarValue(45, num);
                }
              }
            }
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 1000000 & self.MouseData[index1] < 2000000)
          {
            if (self.game.EditObj.se1_modelSelected == self.MouseData[index1] - 1000000)
              self.game.EditObj.se1_modelSelected = -1;
            else
              self.game.EditObj.se1_modelSelected = self.MouseData[index1] - 1000000;
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 10000000 & self.MouseData[index1] < 11000000)
          {
            let mut idValue: i32 = self.MouseData[index1] - 10000000;
            self.game.Data.StringListObj[self.slotQuality].SetData2(0,  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, idValue, 5))), 1, id, 2, 0);
            self.game.EditObj.se1_modelSelected = idValue;
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 11000000 & self.MouseData[index1] < 12000000)
          {
            let mut idValue: i32 = self.MouseData[index1] - 11000000;
            self.game.Data.StringListObj[self.slotQuality].SetData2(0,  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, idValue, 5))), 1, id, 2, 1, true);
            self.game.EditObj.se1_modelSelected = idValue;
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 12000000 & self.MouseData[index1] < 13000000)
          {
            let mut idValue: i32 = self.MouseData[index1] - 12000000;
            self.game.Data.StringListObj[self.slotQuality].SetData2(0,  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, idValue, 5))), 1, id, 2, 2, true);
            self.game.EditObj.se1_modelSelected = idValue;
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 13000000 & self.MouseData[index1] < 14000000)
          {
            let mut idValue: i32 = self.MouseData[index1] - 13000000;
            self.game.Data.StringListObj[self.slotQuality].SetData2(0,  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, idValue, 5))), 1, id, 2, 3, true);
            self.game.EditObj.se1_modelSelected = idValue;
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 14000000 & self.MouseData[index1] < 15000000)
          {
            let mut idValue: i32 = self.MouseData[index1] - 14000000;
            self.game.Data.StringListObj[self.slotQuality].SetData2(0,  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, idValue, 5))), 1, id, 2, 4, true);
            self.game.EditObj.se1_modelSelected = idValue;
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 15000000 & self.MouseData[index1] < 16000000)
          {
            let mut idValue: i32 = self.MouseData[index1] - 15000000;
            self.game.Data.StringListObj[self.slotQuality].SetData2(0,  Math.Round(Conversion.Val(self.game.Data.StringListObj[self.slotModel].GetData(0, idValue, 5))), 1, id, 2, 5, true);
            self.game.EditObj.se1_modelSelected = idValue;
            self.dostuff();
            windowReturnClass1.SetFlag(true);
            windowReturnClass1.AddCommand(4, 67);
            return windowReturnClass1;
          }
          if (self.MouseData[index1] > 20000000 & self.MouseData[index1] < 21000000)
          {
            if (self.game.EditObj.se1_modelSelected != self.MouseData[index1] - 20000000)
              self.game.EditObj.se1_modelSelected = self.MouseData[index1] - 20000000;
            self.game.EditObj.UDSpopupText = "";
            self.formref.Cursor = Cursors.WaitCursor;
            self.game.EditObj.UDSClearInput();
            self.game.EventRelatedObj.SetUDSKey("MODELID", self.game.EditObj.se1_modelSelected);
            self.game.EventRelatedObj.DoCheckSpecificEvent(self.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 564, 0, 0));
            self.formref.Cursor = Cursors.Default;
            self.game.EditObj.PopupValue = 21;
            windowReturnClass1.AddCommand(5, 14);
            self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
            windowReturnClass1.SetFlag(true);
            return windowReturnClass1;
          }
        }
      }
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index10: i32 = 0; index10 <= subPartCounter; index10 += 1)
        {
          if (x > self.SubPartX[index10] & x < self.SubPartX[index10] + self.SubPartW[index10] && y > self.SubPartY[index10] & y < self.SubPartY[index10] + self.SubPartH[index10])
          {
            let mut num1: i32 = self.SubPartID[index10];
            if (num1 == self.opt1)
            {
              if (self.game.EditObj.se1_modelObsoleteHidden == 0)
                self.game.EditObj.se1_modelObsoleteHidden = 1;
              else
                self.game.EditObj.se1_modelObsoleteHidden = 0;
              self.SubPartFlag[index10] = true;
              self.ReCalculate();
              self.dostuff();
              windowReturnClass1.SetFlag(true);
              windowReturnClass1.AddCommand(4, 67);
              return windowReturnClass1;
            }
            if (num1 == self.but0id)
            {
              if (self.game.EditObj.se1_modelView != 0)
              {
                self.game.EditObj.se1_modelView = 0;
                self.SubPartFlag[index10] = true;
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
            }
            else if (num1 == self.but1id)
            {
              if (self.game.EditObj.se1_modelView != 1)
              {
                self.game.EditObj.se1_modelView = 1;
                self.SubPartFlag[index10] = true;
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
            }
            else if (num1 == self.but2id)
            {
              if (self.game.EditObj.se1_modelView != 2)
              {
                self.game.EditObj.se1_modelView = 2;
                self.SubPartFlag[index10] = true;
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
            }
            else if (num1 == self.but3id)
            {
              if (self.game.EditObj.se1_modelView != 3)
              {
                self.game.EditObj.se1_modelView = 3;
                self.SubPartFlag[index10] = true;
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
            }
            else if (num1 == self.but4id)
            {
              if (self.game.EditObj.se1_modelView != 4)
              {
                self.game.EditObj.se1_modelView = 4;
                self.SubPartFlag[index10] = true;
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
            }
            else if (num1 == self.but5id)
            {
              if (self.game.EditObj.se1_modelView != 5)
              {
                self.game.EditObj.se1_modelView = 5;
                self.SubPartFlag[index10] = true;
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
            }
            else
            {
              if (num1 == self.listId)
              {
                let mut num2: i32 = self.SubPartList[index10].Click(x - self.SubPartX[index10], y - self.SubPartY[index10]);
                if (num2 > -1)
                {
                  self.SubPartFlag[index10] = true;
                  self.game.EditObj.se1_modelSHQ = num2;
                  self.dostuff();
                  windowReturnClass1.SetFlag(true);
                  windowReturnClass1.AddCommand(4, 67);
                  return windowReturnClass1;
                }
                if (num2 == -2)
                {
                  self.SubPartFlag[index10] = true;
                  self.game.EditObj.se1_modelSHQ = -1;
                  self.dostuff();
                  windowReturnClass1.SetFlag(true);
                  windowReturnClass1.AddCommand(4, 67);
                  return windowReturnClass1;
                }
                self.SubPartFlag[index10] = true;
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
              if (num1 == self.list2Id)
              {
                let mut num3: i32 = self.SubPartList[index10].Click(x - self.SubPartX[index10], y - self.SubPartY[index10]);
                if (num3 > -1)
                {
                  self.SubPartFlag[index10] = true;
                  self.game.EditObj.se1_modelReinf = num3;
                  self.game.EditObj.se1_modelSelected = -1;
                  self.dostuff();
                  windowReturnClass1.SetFlag(true);
                  windowReturnClass1.AddCommand(4, 67);
                  return windowReturnClass1;
                }
                if (num3 == -2)
                {
                  self.SubPartFlag[index10] = true;
                  self.game.EditObj.se1_modelReinf = -1;
                  self.game.EditObj.se1_modelSelected = -1;
                  self.dostuff();
                  windowReturnClass1.SetFlag(true);
                  windowReturnClass1.AddCommand(4, 67);
                  return windowReturnClass1;
                }
                self.SubPartFlag[index10] = true;
                self.dostuff();
                windowReturnClass1.SetFlag(true);
                windowReturnClass1.AddCommand(4, 67);
                return windowReturnClass1;
              }
            }
            let mut assetButtonCounter: i32 = self.assetButtonCounter;
            for (let mut index11: i32 = 0; index11 <= assetButtonCounter; index11 += 1)
            {
              if (self.assetButton[index11] == self.SubPartID[index10])
              {
                if (self.assetButtonData[index11] >= 51 & self.assetButtonData[index11] < 99)
                {
                  self.game.EditObj.se1_modelPage = self.assetButtonData[index11] - 50;
                  self.dostuff();
                  windowReturnClass1.AddCommand(4, 67);
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (self.assetButtonData[index11] >= 101 & self.assetButtonData[index11] < 149)
                {
                  self.game.EditObj.se1_modelPage2 = self.assetButtonData[index11] - 100;
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

    pub fn PopUpRefresh() => self.dostuff();
  }
}
