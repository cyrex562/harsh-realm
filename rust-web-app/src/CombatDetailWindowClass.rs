// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CombatDetailWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class CombatDetailWindowClass : WindowClass
  {
     okid: i32;
     tbacknr: i32;
     oktextid: i32;
     noteid: i32;
     note2id: i32;
     cloudid: i32;
     Pic1Id: i32;
     TAid: i32;
     Ta2Id: i32;
     DateTime lasttime;
     FromMessage: i32;
     bool ForwardKey;
     bool LastDrawAfterEnd;
     useWidth: i32;
     useHeight: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsList1Id: i32;
     ListClass OptionsList1Obj;
     OptionsList2Id: i32;
     ListClass OptionsList2Obj;
     OptionsList3Id: i32;
     ListClass OptionsList3Obj;
     OptionsList4Id: i32;
     ListClass OptionsList4Obj;
     OptionsList5Id: i32;
     ListClass OptionsList5Obj;
     OptionsList6Id: i32;
     ListClass OptionsList6Obj;
     detailnr: i32;
     detailnr2: i32;
     detailnr3: i32;
     detailnr4: i32;
     detailnr1: i32;

    pub CombatDetailWindowClass(ref tGame: GameClass, thn: i32)
      : base(ref tGame, 1024, 768, 8)
    {
      this.FromMessage = tGame.EditObj.FromMessage;
      this.lasttime = DateAndTime.Now;
      this.dostuff();
      this.detailnr = -1;
      this.detailnr2 = -1;
      this.detailnr3 = -1;
      this.detailnr4 = -1;
      this.detailnr1 = -1;
    }

    pub CombatDetailWindowClass(ref tGame: GameClass, thn: i32, bool OldGui)
      : base(ref tGame, 1024, 768, BackSprite: tGame.BACKGROUND3MARC)
    {
      this.FromMessage = tGame.EditObj.FromMessage;
      this.lasttime = DateAndTime.Now;
      this.dostuff();
      this.detailnr = -1;
      this.detailnr2 = -1;
      this.detailnr3 = -1;
      this.detailnr4 = -1;
      this.detailnr1 = -1;
    }

    pub CombatDetailWindowClass(ref tGame: GameClass, thn: i32, tWidth: i32, tHeight: i32)
      : base(ref tGame, tWidth, tHeight, BackSprite: tGame.BACKGROUND3MARC)
    {
      this.FromMessage = tGame.EditObj.FromMessage;
      this.lasttime = DateAndTime.Now;
      this.useWidth = tWidth;
      this.useHeight = tHeight;
      this.dostuff();
      this.detailnr = -1;
      this.detailnr2 = -1;
      this.detailnr3 = -1;
      this.detailnr4 = -1;
      this.detailnr1 = -1;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            this.SubPartList[index].DescriptInfo(x - this.SubPartX[index], y - this.SubPartY[index]);
            if (Operators.CompareString(this.SubPartList[index].Descript, "", false) > 0)
            {
              this.game.EditObj.TipButton = true;
              this.game.EditObj.TipTitle = "";
              this.game.EditObj.TipText = this.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] > 0)
            this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = this.MouseTitle[index];
          this.game.EditObj.TipText = this.MouseText[index];
          break;
        }
      }
    }

    pub fn dostuff()
    {
      if (this.okid > 0)
      {
        this.RemoveSubPart(this.okid);
        this.okid = 0;
      }
      if (this.TAid > 0)
      {
        this.RemoveSubPart(this.TAid);
        this.TAid = 0;
      }
      if (this.noteid > 0)
      {
        this.RemoveSubPart(this.noteid);
        this.noteid = 0;
      }
      SizeF sizeF = SizeF::new();
      bool flag1;
      if ( this.game.Data.RuleVar[839] >= 1.0)
      {
        flag1 = true;
        this.NewBackGroundAndClearAll(this.useWidth, this.useHeight, -1);
      }
      else
      {
        flag1 = false;
        this.NewBackGroundAndClearAll(this.useWidth, this.useHeight, this.game.BACKGROUND3MARC);
      }
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      if (flag1)
        DrawMod.DrawMessFrame(ref this.OwnBitmap, ref graphics, 0, 0, this.useWidth, this.useHeight);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      DrawMod.DrawTextColouredMarc(ref graphics, "COMBAT ROUNDS", this.game.MarcFont4, 10, 8, Color.White);
      this.OptionsListObj = ListClass::new();
      if (this.detailnr > this.game.TempCombat.CombatRound)
        this.detailnr = -1;
      let mut tlistselect1: i32 =  -1;
      let mut num1: i32 =  -1;
      if (this.game.TempCombat.CombatRound > -1)
      {
        let mut combatRound: i32 =  this.game.TempCombat.CombatRound;
        for (let mut index: i32 =  1; index <= combatRound; index += 1)
        {
          num1 += 1;
          if (this.detailnr <= 0)
            this.detailnr = index;
          if (this.detailnr == index)
            tlistselect1 = num1;
          this.OptionsListObj.add("Round " + Strings.Trim(Conversion.Str( index)), index);
        }
        if (this.OptionsListId > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, tlistselect1);
          this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
        }
        else
        {
          ListClass optionsListObj = this.OptionsListObj;
          let mut tlistselect2: i32 =  tlistselect1;
          let mut game: GameClass = this.game;
          ref local1: Bitmap = ref this.OwnBitmap;
          let mut num2: i32 =  flag1 ? 1 : 0;
          font: Font =  null;
          ref local2: Font = ref font;
          let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsListObj, 13, 280, tlistselect2, game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: (ref local1), bbx: 10, bby: 25, tMarcStyle: (num2 != 0), overruleFont: (ref local2));
          this.OptionsListId = this.AddSubPart(ref tsubpart, 10, 25, 280, 224, 0);
        }
      }
      DrawMod.DrawTextColouredMarc(ref graphics, "UNITS", this.game.MarcFont4, 10, 252, Color.White);
      this.OptionsList1Obj = ListClass::new();
      if (this.detailnr1 > this.game.TempCombat.UCounter)
        this.detailnr1 = -1;
      let mut tlistselect3: i32 =  -1;
      let mut num3: i32 =  -1;
      if (this.game.TempCombat.UCounter > -1)
      {
        let mut ucounter: i32 =  this.game.TempCombat.UCounter;
        for (let mut tdata: i32 =  0; tdata <= ucounter; tdata += 1)
        {
          bool flag2 = true;
          bool flag3 = true;
          if ( this.game.Data.RuleVar[431] > 0.0)
          {
            flag2 = false;
            let mut icounter: i32 =  this.game.TempCombat.ICounter;
            for (let mut index: i32 =  0; index <= icounter; index += 1)
            {
              if (this.game.TempCombat.IList[index].IUlistNr == tdata)
              {
                if (this.game.TempCombat.IList[index].IvisibleFromRound < this.game.TempCombat.CombatRound | this.game.Data.UnitObj[this.game.TempCombat.IList[index].IUnr].Regime == this.game.Data.Turn)
                {
                  flag2 = true;
                  flag3 = false;
                }
                else if (!this.game.Data.FOWOn)
                  flag2 = true;
              }
            }
          }
          if (flag2)
          {
            num3 += 1;
            if (this.detailnr1 == -1)
              this.detailnr1 = tdata;
            if (this.detailnr1 == tdata)
              tlistselect3 = num3;
            str: String = this.game.Data.UnitObj[this.game.TempCombat.UList[tdata].UNr].Name;
            if ( this.game.Data.RuleVar[431] > 0.0 & this.game.Data.FOWOn &&  this.game.HandyFunctionsObj.CanWeSeeUnit(this.game.TempCombat.UList[tdata].UNr, this.game.Data.Turn) <  this.game.Data.RuleVar[55])
              str = "Unknown Unit";
            if (flag3)
              str += " [Hidden]";
            this.OptionsList1Obj.add(Strings.Left(str, 30), tdata, Strings.Left(this.game.Data.RegimeObj[this.game.Data.UnitObj[this.game.TempCombat.UList[tdata].UNr].Regime].Name, 20));
          }
        }
        if (this.OptionsList1Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList1Id)].Refresh(this.OptionsList1Obj, tlistselect3);
          this.SubPartFlag[this.SubpartNr(this.OptionsList1Id)] = true;
        }
        else
        {
          ListClass optionsList1Obj = this.OptionsList1Obj;
          let mut tlistselect4: i32 =  tlistselect3;
          let mut game: GameClass = this.game;
          ref local3: Bitmap = ref this.OwnBitmap;
          let mut num4: i32 =  flag1 ? 1 : 0;
          font: Font =  null;
          ref local4: Font = ref font;
          let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsList1Obj, 14, 280, tlistselect4, game, tHeaderCenter: false, tShowPair: true, tValueWidth: 120, tdotopandbottom: false, tbackbitmap: (ref local3), bbx: 10, bby: 270, tMarcStyle: (num4 != 0), overruleFont: (ref local4));
          this.OptionsList1Id = this.AddSubPart(ref tsubpart, 10, 270, 280, 240, 0);
        }
      }
      bool flag4 = true;
      if ( this.game.Data.RuleVar[431] > 0.0)
      {
        flag4 = false;
        let mut icounter: i32 =  this.game.TempCombat.ICounter;
        for (let mut index: i32 =  0; index <= icounter; index += 1)
        {
          if (this.game.TempCombat.IList[index].IUlistNr == this.detailnr1 && this.game.TempCombat.IList[index].IvisibleFromRound < this.detailnr | this.game.Data.UnitObj[this.game.TempCombat.IList[index].IUnr].Regime == this.game.Data.Turn)
            flag4 = true;
        }
      }
      if (flag4)
      {
        DrawMod.DrawTextColouredMarc(ref graphics, "UNIT REPORTS", this.game.MarcFont4, 10, 517, Color.White);
        this.OptionsList2Obj = ListClass::new();
        let mut tlistselect5: i32 =  -1;
        if (this.detailnr1 > -1)
        {
          let mut num5: i32 =  -1;
          let mut repCounter: i32 =  this.game.TempCombat.RepCounter;
          num6: i32;
          for (let mut tdata: i32 =  0; tdata <= repCounter; tdata += 1)
          {
            if (this.game.TempCombat.RepFrom[tdata] == this.detailnr1 & this.game.TempCombat.RepRound[tdata] == this.detailnr)
            {
              num5 += 1;
              if (num5 == 0)
                num6 = tdata;
              if (this.detailnr2 == tdata)
                tlistselect5 = num5;
              this.OptionsList2Obj.add(Strings.Left(this.game.TempCombat.RepTitle[tdata], 55), tdata);
            }
          }
          if (tlistselect5 == -1)
            this.detailnr2 = -1;
          if (tlistselect5 == -1)
            this.detailnr2 = -1;
          if (num5 > -1 & this.detailnr4 == -1 & tlistselect5 == -1)
          {
            tlistselect5 = 0;
            this.detailnr2 = num6;
          }
        }
        if (this.OptionsList2Id > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsList2Id)].Refresh(this.OptionsList2Obj, tlistselect5);
          this.SubPartFlag[this.SubpartNr(this.OptionsList2Id)] = true;
        }
        else
        {
          ListClass optionsList2Obj = this.OptionsList2Obj;
          let mut tlistselect6: i32 =  tlistselect5;
          let mut game: GameClass = this.game;
          ref local5: Bitmap = ref this.OwnBitmap;
          let mut num7: i32 =  flag1 ? 1 : 0;
          font: Font =  null;
          ref local6: Font = ref font;
          let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsList2Obj, 11, 280, tlistselect6, game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: (ref local5), bbx: 10, bby: 535, tMarcStyle: (num7 != 0), overruleFont: (ref local6));
          this.OptionsList2Id = this.AddSubPart(ref tsubpart, 10, 535, 280, 192, 0);
        }
      }
      DrawMod.DrawTextColouredMarc(ref graphics, "INDIVIDUAL LIST", this.game.MarcFont4, 310, 8, Color.White);
      this.OptionsList3Obj = ListClass::new();
      let mut tlistselect7: i32 =  -1;
      if (this.detailnr1 > -1)
      {
        let mut num8: i32 =  -1;
        let mut icounter: i32 =  this.game.TempCombat.ICounter;
        for (let mut tdata: i32 =  0; tdata <= icounter; tdata += 1)
        {
          if (this.game.TempCombat.IList[tdata].IUnr == this.game.TempCombat.UList[this.detailnr1].UNr)
          {
            bool flag5 = true;
            bool flag6 = false;
            if ( this.game.Data.RuleVar[431] > 0.0)
            {
              flag5 = false;
              if (this.game.TempCombat.IList[tdata].IvisibleFromRound < this.detailnr | this.game.Data.UnitObj[this.game.TempCombat.IList[tdata].IUnr].Regime == this.game.Data.Turn)
                flag5 = true;
              else if (!this.game.Data.FOWOn)
              {
                flag5 = true;
                flag6 = true;
              }
            }
            if (flag5)
            {
              num8 += 1;
              if (this.detailnr3 == -1)
                this.detailnr3 = tdata;
              if (this.detailnr3 == tdata)
                tlistselect7 = num8;
              tvalue: String;
              tvalue2: String;
              if (!Information.IsNothing( this.game.TempCombat.IList[tdata].IHistoricState))
              {
                tvalue = this.game.TempCombat.IList[tdata].IHistoricState[this.detailnr];
                tvalue2 = this.game.TempCombat.IList[tdata].IHistoricState2[this.detailnr];
              }
              else
              {
                tvalue = "";
                tvalue2 = "";
              }
              str1: String = this.game.Data.SFTypeObj[this.game.TempCombat.IList[tdata].ISFType].Name;
              Left: String = "";
              if (flag6)
                str1 += " [Hidden]";
              if (!Information.IsNothing( this.game.TempCombat.IList[tdata].IunitFeat) && this.game.TempCombat.IList[tdata].IunitFeat.Counter > -1)
              {
                if (this.game.TempCombat.IList[tdata].IunitFeatStart > 0 & !Information.IsNothing( this.game.TempCombat.customCombatObj) & this.game.TempCombat.IList[tdata].IunitFeat.Data1[0] == this.game.TempCombat.IList[tdata].IID && this.game.TempCombat.IList[tdata].IunitFeatDeadRound >= this.detailnr | this.game.TempCombat.IList[tdata].IunitFeatDeadRound < 1)
                  Left = " + " + this.game.TempCombat.customCombatObj.GetUnitFeatName(this.game, this.game.TempCombat.IList[tdata].IunitFeatStart) + "*";
                str2: String = "";
                if (!Information.IsNothing( this.game.TempCombat.customCombatObj))
                {
                  let mut num9: i32 =  0;
                  let mut counter: i32 =  this.game.TempCombat.IList[tdata].IunitFeat.Counter;
                  for (let mut index: i32 =  0; index <= counter; index += 1)
                  {
                    if (this.game.TempCombat.IList[this.game.TempCombat.FindISlot(this.game.TempCombat.IList[tdata].IunitFeat.Data1[index])].IunitFeatDeadRound >= this.detailnr | this.game.TempCombat.IList[this.game.TempCombat.FindISlot(this.game.TempCombat.IList[tdata].IunitFeat.Data1[index])].IunitFeatDeadRound < 1 && this.game.TempCombat.IList[tdata].IunitFeat.Data1[index] != this.game.TempCombat.IList[tdata].IID && this.game.TempCombat.IList[this.game.TempCombat.FindISlot(this.game.TempCombat.IList[tdata].IunitFeat.Data1[index])].IKilled >= this.detailnr | this.game.TempCombat.IList[this.game.TempCombat.FindISlot(this.game.TempCombat.IList[tdata].IunitFeat.Data1[index])].IKilled < 1 && this.game.TempCombat.IList[this.game.TempCombat.FindISlot(this.game.TempCombat.IList[tdata].IunitFeat.Data1[index])].IRetreated >= this.detailnr | this.game.TempCombat.IList[this.game.TempCombat.FindISlot(this.game.TempCombat.IList[tdata].IunitFeat.Data1[index])].IRetreated < 1)
                    {
                      num9 += 1;
                      if (num9 == 1)
                        str2 = this.game.TempCombat.customCombatObj.GetUnitFeatName(this.game, this.game.TempCombat.IList[tdata].IunitFeat.Id[index]);
                    }
                  }
                  if (num9 > 1 & Operators.CompareString(Left, "", false) != 0)
                    str1 = str1 + Left + " + " + num9.ToString() + " others";
                  else if (num9 > 0 & Operators.CompareString(Left, "", false) == 0)
                  {
                    str1 = str1 + " + " + str2;
                    let mut num10: i32 =  num9 - 1;
                    if (num10 >= 1)
                      str1 = str1 + " + " + num10.ToString() + " others";
                  }
                  else
                    str1 += Left;
                }
              }
              this.OptionsList3Obj.add(Strings.Left(Strings.Trim(Conversion.Str( this.game.TempCombat.IList[tdata].IID)) + ") " + str1, 30), tdata, tvalue, tvalue2);
            }
          }
        }
        if (tlistselect7 == -1)
          this.detailnr3 = -1;
      }
      if (this.OptionsList3Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, tlistselect7);
        this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
      }
      else
      {
        let mut num11: i32 =   Math.Round( (this.useHeight - 768) / 16.0);
        ListClass optionsList3Obj = this.OptionsList3Obj;
        let mut tlistsize: i32 =  36 + num11;
        let mut tlistselect8: i32 =  tlistselect7;
        let mut game: GameClass = this.game;
        ref local7: Bitmap = ref this.OwnBitmap;
        let mut num12: i32 =  flag1 ? 1 : 0;
        font: Font =  null;
        ref local8: Font = ref font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsList3Obj, tlistsize, 280, tlistselect8, game, tHeaderCenter: false, tShowPair: true, tValueWidth: 140, tdotopandbottom: false, tbackbitmap: (ref local7), bbx: 310, bby: 25, tMarcStyle: (num12 != 0), overruleFont: (ref local8));
        this.OptionsList3Id = this.AddSubPart(ref tsubpart, 310, 25, 280, (37 + num11) * 16, 0);
      }
      DrawMod.DrawTextColouredMarc(ref graphics, "INDIVIDUAL REPORTS", this.game.MarcFont4, 610, 8, Color.White);
      this.OptionsList4Obj = ListClass::new();
      let mut tlistselect9: i32 =  -1;
      if (this.detailnr3 > -1)
      {
        let mut num13: i32 =  -1;
        let mut repCounter: i32 =  this.game.TempCombat.RepCounter;
        num14: i32;
        for (let mut tdata: i32 =  0; tdata <= repCounter; tdata += 1)
        {
          if (this.game.TempCombat.RepFrom[tdata] == this.detailnr3 + 10000 & this.game.TempCombat.RepRound[tdata] == this.detailnr)
          {
            num13 += 1;
            if (num13 == 0)
              num14 = tdata;
            if (this.detailnr4 == tdata)
              tlistselect9 = num13;
            this.OptionsList4Obj.add(Strings.Left(this.game.TempCombat.RepTitle[tdata], 85), tdata);
          }
        }
        if (tlistselect9 == -1)
          this.detailnr4 = -1;
        if (this.detailnr2 == -1 & num13 > -1 & tlistselect9 == -1)
        {
          tlistselect9 = 0;
          this.detailnr4 = num14;
        }
      }
      let mut num15: i32 =  Math.Min(300, this.useWidth - 1024);
      let mut num16: i32 =   Math.Round( num15 / 2.0);
      if (this.OptionsList4Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList4Id)].Refresh(this.OptionsList4Obj, tlistselect9);
        this.SubPartFlag[this.SubpartNr(this.OptionsList4Id)] = true;
      }
      else
      {
        ListClass optionsList4Obj = this.OptionsList4Obj;
        let mut twidth: i32 =  380 + num15;
        let mut tlistselect10: i32 =  tlistselect9;
        let mut game: GameClass = this.game;
        ref local9: Bitmap = ref this.OwnBitmap;
        let mut num17: i32 =  flag1 ? 1 : 0;
        font: Font =  null;
        ref local10: Font = ref font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(optionsList4Obj, 15, twidth, tlistselect10, game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: (ref local9), bbx: 610, bby: 25, tMarcStyle: (num17 != 0), overruleFont: (ref local10));
        this.OptionsList4Id = this.AddSubPart(ref tsubpart, 610, 25, 380 + num15, 256, 0);
      }
      this.RemoveSubPart(this.TAid);
      this.RemoveSubPart(this.OptionsList5Id);
      this.OptionsList5Id = 0;
      this.RemoveSubPart(this.OptionsList6Id);
      this.OptionsList6Id = 0;
      index1: i32;
      if (this.detailnr2 > -1)
        index1 = this.detailnr2;
      if (this.detailnr4 > -1)
        index1 = this.detailnr4;
      SubPartClass tsubpart1;
      if (index1 > -1)
      {
        str: String = Strings.UCase(this.game.TempCombat.RepTitle[index1]) + "\r\n" + "\r\n" + this.game.TempCombat.RepText[index1];
        if (Strings.InStr(str, "[tab]") == 0 & this.detailnr2 == index1)
          str = "[tab]Unit Report," + str + "[/tab]";
        if (Strings.InStr(str, "[tab]") == 0 & this.detailnr4 == index1)
          str = "[tab]Individual Report," + str + "[/tab]";
        if (this.game.TempCombat.RepType[index1] == 1)
        {
          if (flag1)
          {
            tsubpart1 =  new TextAreaClass2(this.game, 380 + num15, 6, this.game.MarcFont5, str, 15, ref this.OwnBitmap, 610, 300);
            this.TAid = this.AddSubPart(ref tsubpart1, 610, 300, 380 + num15, 105, 0);
          }
          else
          {
            let mut tsubpart2: SubPartClass =  new TextAreaClass(this.game, 380 + num15, 6, this.game.gamefont1b, "", false, str, Color.White, tItemSize: 15, tbackbitmap: (ref this.OwnBitmap), bbx: 610, bby: 300);
            this.TAid = this.AddSubPart(ref tsubpart2, 610, 300, 380 + num15, 105, 0);
          }
          DrawMod.DrawTextColouredMarc(ref graphics, "ATTACK SCORE MODS", this.game.MarcFont4, 610, 447, Color.White);
          this.OptionsList5Obj = ListClass::new();
          let mut index2: i32 =  1;
          do
          {
            if (!Information.IsNothing( this.game.TempCombat.RepCom[index2, 0, index1]) && this.game.TempCombat.RepCom[index2, 0, index1].Length > 0 & (index2 == 1 | Operators.CompareString(this.game.TempCombat.RepCom[index2 - 1, 2, index1], this.game.TempCombat.RepCom[index2, 2, index1], false) != 0 | Operators.CompareString(this.game.TempCombat.RepCom[index2, 0, index1], "After mods", false) == 0) && Operators.CompareString(this.game.TempCombat.RepCom[index2, 2, index1], "?", false) != 0)
              this.OptionsList5Obj.add(this.game.TempCombat.RepCom[index2, 0, index1], -1, this.game.TempCombat.RepCom[index2, 1, index1], Conversions.ToString(Math.Round( Conversions.ToSingle(this.game.TempCombat.RepCom[index2, 2, index1]), 1)));
            index2 += 1;
          }
          while (index2 <= 60);
          let mut num18: i32 =   Math.Round( (this.useWidth - 1024) / 5.0);
          ListClass optionsList5Obj = this.OptionsList5Obj;
          let mut twidth1: i32 =  170 + num16;
          let mut tlistselect11: i32 =  tlistselect9;
          let mut game1: GameClass = this.game;
          let mut tValueWidth1: i32 =  80 + num18;
          ref local11: Bitmap = ref this.OwnBitmap;
          let mut num19: i32 =  flag1 ? 1 : 0;
          font1: Font =  null;
          ref local12: Font = ref font1;
          tsubpart1 =  new ListSubPartClass(optionsList5Obj, 15, twidth1, tlistselect11, game1, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: tValueWidth1, tdotopandbottom: false, tbackbitmap: (ref local11), bbx: 610, bby: 465, tMarcStyle: (num19 != 0), overruleFont: (ref local12));
          this.OptionsList5Id = this.AddSubPart(ref tsubpart1, 610, 465, 170 + num16, 256, 0);
          DrawMod.DrawTextColouredMarc(ref graphics, "DEFENDER SCORE MODS", this.game.MarcFont4, 810 + num16, 447, Color.White);
          this.OptionsList6Obj = ListClass::new();
          let mut index3: i32 =  1;
          do
          {
            if (!Information.IsNothing( this.game.TempCombat.RepCom[index3, 3, index1]) & !Information.IsNothing( this.game.TempCombat.RepCom[index3, 5, index1]) && Operators.CompareString(this.game.TempCombat.RepCom[index3, 5, index1], "?", false) != 0 & this.game.TempCombat.RepCom[index3, 3, index1].Length > 0 & (index3 == 1 | Operators.CompareString(this.game.TempCombat.RepCom[index3 - 1, 5, index1], this.game.TempCombat.RepCom[index3, 5, index1], false) != 0 | Operators.CompareString(this.game.TempCombat.RepCom[index3, 3, index1], "After mods", false) == 0) && Operators.CompareString(this.game.TempCombat.RepCom[index3, 5, index1], "?", false) != 0)
              this.OptionsList6Obj.add(this.game.TempCombat.RepCom[index3, 3, index1], -1, this.game.TempCombat.RepCom[index3, 4, index1], Conversions.ToString(Math.Round( Conversions.ToSingle(this.game.TempCombat.RepCom[index3, 5, index1]), 1)));
            index3 += 1;
          }
          while (index3 <= 60);
          let mut num20: i32 =   Math.Round( (this.useWidth - 1024) / 5.0);
          ListClass optionsList6Obj = this.OptionsList6Obj;
          let mut twidth2: i32 =  170 + num16;
          let mut tlistselect12: i32 =  tlistselect9;
          let mut game2: GameClass = this.game;
          let mut tValueWidth2: i32 =  80 + num20;
          ref local13: Bitmap = ref this.OwnBitmap;
          let mut bbx: i32 =  810 + num16;
          let mut num21: i32 =  flag1 ? 1 : 0;
          font2: Font =  null;
          ref local14: Font = ref font2;
          tsubpart1 =  new ListSubPartClass(optionsList6Obj, 15, twidth2, tlistselect12, game2, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: tValueWidth2, tdotopandbottom: false, tbackbitmap: (ref local13), bbx: bbx, bby: 465, tMarcStyle: (num21 != 0), overruleFont: (ref local14));
          this.OptionsList6Id = this.AddSubPart(ref tsubpart1, 810 + num16, 465, 170 + num16, 256, 0);
        }
        else if (this.game.TempCombat.RepType[index1] == 2)
        {
          let mut num22: i32 =  Math.Min(300, this.useHeight - 768);
          DrawMod.DrawTextColouredMarc(ref graphics, "STATUS", this.game.MarcFont4, 610, 300, Color.White);
          this.OptionsList5Obj = ListClass::new();
          let mut index4: i32 =  1;
          do
          {
            if (!Information.IsNothing( this.game.TempCombat.RepCom[index4, 0, index1]) && this.game.TempCombat.RepCom[index4, 0, index1].Length > 0)
              this.OptionsList5Obj.add(this.game.TempCombat.RepCom[index4, 0, index1], -1, this.game.TempCombat.RepCom[index4, 1, index1]);
            index4 += 1;
          }
          while (index4 <= 60);
          let mut num23: i32 =   Math.Round( num22 / 16.0);
          ListClass optionsList5Obj = this.OptionsList5Obj;
          let mut tlistsize: i32 =  19 + num23;
          let mut tlistselect13: i32 =  tlistselect9;
          let mut game: GameClass = this.game;
          ref local15: Bitmap = ref this.OwnBitmap;
          let mut num24: i32 =  flag1 ? 1 : 0;
          font: Font =  null;
          ref local16: Font = ref font;
          let mut tsubpart3: SubPartClass =  new ListSubPartClass(optionsList5Obj, tlistsize, 380, tlistselect13, game, tHeaderCenter: false, tHighlight: false, tShowPair: true, tValueWidth: 200, tdotopandbottom: false, tbackbitmap: (ref local15), bbx: 610, bby: 317, tMarcStyle: (num24 != 0), overruleFont: (ref local16));
          this.OptionsList5Id = this.AddSubPart(ref tsubpart3, 610, 317, 380, (20 + num23) * 16, 0);
        }
        else if (flag1)
        {
          let mut tsubpart4: SubPartClass =  new TextAreaClass2(this.game, 380, 26, this.game.MarcFont5, str, 15, ref this.OwnBitmap, 610, 300);
          this.TAid = this.AddSubPart(ref tsubpart4, 610, 300, 380, 405, 0);
        }
        else
        {
          tsubpart1 =  new TextAreaClass(this.game, 380, 26, this.game.gamefont1b, "", false, str, Color.White, tItemSize: 15, tbackbitmap: (ref this.OwnBitmap), bbx: 610, bby: 300);
          this.TAid = this.AddSubPart(ref tsubpart1, 610, 300, 380, 405, 0);
        }
      }
      combatOneSentence: String = this.game.EditObj.CombatOneSentence;
      tText: String = this.game.EditObj.CombatOneSentenceCustom.Length <= 0 ? this.game.EditObj.CombatOneSentenceCustom : combatOneSentence + " " + this.game.EditObj.CombatOneSentenceCustom;
      if (tText.Length > 1)
      {
        if (flag1)
        {
          tsubpart1 =  new TextAreaClass2(this.game, 290, 3, this.game.MarcFont5, tText, 15, ref this.OwnBitmap, 310, 620);
          this.Ta2Id = this.AddSubPart(ref tsubpart1, 310, 620, 290, 60, 0);
        }
        else
        {
          tsubpart1 =  new TextAreaClass(this.game, 290, 3, this.game.gamefont1b, "", false, tText, Color.White, tItemSize: 15, tbackbitmap: (ref this.OwnBitmap), bbx: 310, bby: 620);
          this.Ta2Id = this.AddSubPart(ref tsubpart1, 310, 620, 290, 60, 0);
        }
      }
      let mut num25: i32 =   Math.Round( this.useWidth / 2.0 - 50.0);
      if (flag1)
      {
        tsubpart1 =  new TextButtonPartClass("BACK", 100, "Click to return to regular combat screen\r\nOr press any key instead.", ref this.OwnBitmap, num25, this.useHeight - 40, theight: 20, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.okid = this.AddSubPart(ref tsubpart1, num25, this.useHeight - 40, 100, 20, 1);
      }
      else
      {
        tsubpart1 =  new TextButtonPartClass("Back", 100, tBackbitmap: (ref this.OwnBitmap), bbx: 460, bby: (this.useHeight - 70));
        this.okid = this.AddSubPart(ref tsubpart1, 460, this.useHeight - 70, 100, 35, 1);
      }
    }

    pub fn GetSfNrBitmap(sfnr: i32, att: i32) -> i32
    {
      let mut type: i32 =  this.game.Data.SFObj[sfnr].Type;
      let mut symbolSpriteId: i32 =  this.game.Data.SFTypeObj[type].SymbolSpriteID;
      let mut index1: i32 =  -1;
      if (att == 0)
        index1 = this.game.TempCombat.DefenderRegime;
      if (att == 1)
        index1 = this.game.TempCombat.AttackerRegime;
      if (index1 > -1)
      {
        if (this.game.Data.RegimeObj[index1].ExtraGraphicUse > -1)
        {
          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[type].ExtraCounter;
          for (let mut index2: i32 =  0; index2 <= extraCounter; index2 += 1)
          {
            if (this.game.Data.SFTypeObj[type].ExtraCode[index2] == this.game.Data.RegimeObj[index1].ExtraGraphicUse)
              symbolSpriteId = this.game.Data.SFTypeObj[type].ExtraSymbolSpriteID[index2];
          }
        }
        else
        {
          index3: i32;
          if (this.game.Data.PeopleObj[this.game.Data.SFObj[this.game.TempCombat.IList[index3].ISFNr].People].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 =  this.game.Data.SFTypeObj[type].ExtraCounter;
            for (let mut index4: i32 =  0; index4 <= extraCounter; index4 += 1)
            {
              if (this.game.Data.SFTypeObj[type].ExtraCode[index4] == this.game.Data.PeopleObj[this.game.Data.SFObj[this.game.TempCombat.IList[index3].ISFNr].People].ExtraGraphicUse)
                symbolSpriteId = this.game.Data.SFTypeObj[type].ExtraSymbolSpriteID[index4];
            }
          }
        }
      }
      return symbolSpriteId;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num1: i32 =  this.SubPartID[index];
            if (num1 == this.TAid)
            {
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsListId)
            {
              let mut num2: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num2 > -1)
              {
                this.detailnr = num2;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList1Id)
            {
              let mut num3: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num3 > -1)
              {
                this.detailnr1 = num3;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList2Id)
            {
              let mut num4: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num4 > -1)
              {
                this.detailnr2 = num4;
                this.detailnr4 = -1;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList3Id)
            {
              let mut num5: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num5 > -1)
              {
                this.detailnr3 = num5;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.OptionsList4Id)
            {
              let mut num6: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num6 > -1)
              {
                this.detailnr4 = num6;
                this.detailnr2 = -1;
                this.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.okid)
            {
              if ( this.game.Data.RuleVar[839] == 1.0)
              {
                this.game.EditObj.PopupValue = 7;
                windowReturnClass.AddCommand(5, 14);
              }
              else
              {
                windowReturnClass.AddCommand(1, 84);
                windowReturnClass.AddCommand(2, 84);
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
