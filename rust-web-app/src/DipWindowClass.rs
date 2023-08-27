// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.DipWindowClass
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
  pub class DipWindowClass : WindowClass
  {
     LocNr: i32;
     BNameId: i32;
     BNameTextId: i32;
     B1Id: i32;
     Tab1id: i32;
     tab2id: i32;
     tab3id: i32;
     F1id: i32;
     F2id: i32;
     F3id: i32;
     F4id: i32;
     F5id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     OptionsListId: i32;
     ATListClass OptionsListObj;
     OptionsList2Id: i32;
     ATListClass OptionsList2Obj;
     OptionsList3Id: i32;
     ATListClass OptionsList3Obj;
     OptionsList4Id: i32;
     ATListClass OptionsList4Obj;
     OptionsList5Id: i32;
     ATListClass OptionsList5Obj;
     detailnr: i32;
     regnr: i32;
     tabsheet: i32;

    pub DipWindowClass(ref tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 =  -1, let mut sy: i32 =  -1)
      : base(ref tGame, 1024, 200, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      this.fixshade = true;
      this.regnr = this.game.Data.Turn;
      this.detailnr = -1;
      if (this.game.SelectX > -1)
        this.detailnr = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime;
      this.dostuff();
    }

    pub fn DoRefresh()
    {
      if (this.game.EditObj.CurrentMiniX > -1 & this.game.EditObj.CurrentMiniY > -1 & this.game.EditObj.MapSelected > -1)
      {
        if (this.game.EditObj.CurrentMiniX <= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth & this.game.EditObj.CurrentMiniY <= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
        {
          this.detailnr = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.CurrentMiniX, this.game.EditObj.CurrentMiniY].Regime;
          if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.CurrentMiniX, this.game.EditObj.CurrentMiniY].get_SeeNow(this.game.Data.Turn) < 1)
            this.detailnr = -1;
        }
        else
          this.detailnr = -1;
      }
      else
        this.detailnr = -1;
      this.dostuff();
    }

     void dostuff()
    {
      if (this.Text1Id > 0)
        this.RemoveSubPart(this.Text1Id);
      if (this.Text2Id > 0)
        this.RemoveSubPart(this.Text2Id);
      if (this.Text3Id > 0)
        this.RemoveSubPart(this.Text3Id);
      if (this.B1Id > 0)
        this.RemoveSubPart(this.B1Id);
      if (this.B1TextId > 0)
        this.RemoveSubPart(this.B1TextId);
      if (this.B2Id > 0)
        this.RemoveSubPart(this.B2Id);
      if (this.B3Id > 0)
        this.RemoveSubPart(this.B3Id);
      if (this.B4Id > 0)
        this.RemoveSubPart(this.B4Id);
      if (this.B5Id > 0)
        this.RemoveSubPart(this.B5Id);
      if (this.B2TextId > 0)
        this.RemoveSubPart(this.B2TextId);
      if (this.F1id > 0)
        this.RemoveSubPart(this.F1id);
      if (this.F2id > 0)
        this.RemoveSubPart(this.F2id);
      if (this.F3id > 0)
        this.RemoveSubPart(this.F3id);
      if (this.F4id > 0)
        this.RemoveSubPart(this.F4id);
      if (this.F5id > 0)
        this.RemoveSubPart(this.F5id);
      if (this.Tab1id > 0)
        this.RemoveSubPart(this.Tab1id);
      if (this.tab2id > 0)
        this.RemoveSubPart(this.tab2id);
      if (this.tab3id > 0)
        this.RemoveSubPart(this.tab3id);
      this.B1Id = 0;
      this.B2Id = 0;
      this.B3Id = 0;
      this.B4Id = 0;
      this.B5Id = 0;
      this.NewBackGroundAndClearAll(1024, 200, -1);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      this.OptionsListObj = ATListClass::new();
      if (this.detailnr > this.game.Data.RegimeCounter)
        this.detailnr = -1;
      let mut tlistselect1: i32 =  -1;
      let mut num1: i32 =  -1;
      SubPartClass tsubpart;
      if (this.game.Data.RegimeCounter > -1)
      {
        let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
        for (let mut tdata: i32 =  0; tdata <= regimeCounter; tdata += 1)
        {
          if (!this.game.Data.RegimeObj[tdata].Sleep | !this.game.Data.RegimeObj[tdata].DipBlock)
          {
            num1 += 1;
            if (this.detailnr == tdata)
              tlistselect1 = num1;
            this.OptionsListObj.add(this.game.Data.RegimeObj[tdata].Name, tdata, tbmp: BitmapStore.GetBitmap(this.game.Data.RegimeObj[tdata].HQSpriteNr, -1), tr: this.game.Data.RegimeObj[tdata].Red, tg: this.game.Data.RegimeObj[tdata].Green, tb: this.game.Data.RegimeObj[tdata].Blue);
          }
        }
        if (this.OptionsListId > 0)
        {
          this.SubPartList[this.SubpartNr(this.OptionsListId)].Refresh(this.OptionsListObj, tlistselect1);
          this.SubPartFlag[this.SubpartNr(this.OptionsListId)] = true;
        }
        else
        {
          tsubpart =  new ATListSubPartClass(this.OptionsListObj, 7, 195, tlistselect1, this.game, true, "Regimes", tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 200, bby: 10);
          this.OptionsListId = this.AddSubPart(ref tsubpart, 200, 10, 195, 128, 0);
        }
      }
      this.OptionsList2Obj = ATListClass::new();
      let mut num2: i32 =  -1;
      let mut tlistselect2: i32 =  -1;
      if (this.game.Data.VPWin > 0)
        this.OptionsList2Obj.add("VP Win", -1, Conversion.Str( this.game.Data.VPWin));
      let mut tdata1: i32 =  0;
      do
      {
        if (this.game.Data.GameSlotShow[tdata1])
          this.OptionsList2Obj.add(this.game.Data.GameSlotName[tdata1], tdata1, Conversion.Str( this.game.Data.GameSlot[tdata1]));
        tdata1 += 1;
      }
      while (tdata1 <= 499);
      if (this.OptionsList2Id > 0)
      {
        this.SubPartList[this.SubpartNr(this.OptionsList2Id)].Refresh(this.OptionsList2Obj, tlistselect2);
        this.SubPartFlag[this.SubpartNr(this.OptionsList2Id)] = true;
      }
      else
      {
        tsubpart =  new ATListSubPartClass(this.OptionsList2Obj, 7, 195, tlistselect2, this.game, true, "Game Variables", tHighlight: false, tShowPair: true, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 10, bby: 10);
        this.OptionsList2Id = this.AddSubPart(ref tsubpart, 0, 10, 195, 128, 0);
      }
      tsubpart =  new TextButtonPartClass("General", 100, "Click to view general info on selected regime", ref this.OwnBitmap, 427, 10, tred: (this.tabsheet == 0));
      this.Tab1id = this.AddSubPart(ref tsubpart, 427, 10, 100, 35, 1);
      tsubpart =  new TextButtonPartClass("Diplomatic", 100, "Click to view the diplomatic status the selected regime has with the other regimes", ref this.OwnBitmap, 427, 50, tred: (this.tabsheet == 1));
      this.tab2id = this.AddSubPart(ref tsubpart, 427, 50, 100, 35, 1);
      tsubpart =  new TextButtonPartClass("Resources", 100, "Click to view political points and resources the selected regime has", ref this.OwnBitmap, 427, 90, tred: (this.tabsheet == 2));
      this.tab3id = this.AddSubPart(ref tsubpart, 427, 90, 100, 35, 1);
      if (this.detailnr > -1)
      {
        tvalue: String;
        if (this.tabsheet == 0)
        {
          this.OptionsList3Obj = ATListClass::new();
          this.OptionsList3Obj.add("Name", -1, this.game.Data.RegimeObj[this.detailnr].Name);
          this.OptionsList3Obj.add("People", -1, this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.detailnr].People].Name);
          this.OptionsList3Obj.add("BaseMor", -1, Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[this.detailnr].BaseMorale)));
          tvalue = Strings.Trim(Conversion.Str( this.game.HandyFunctionsObj.GetRegimeVP(this.detailnr)));
          this.OptionsList3Obj.add("VP", -1, tvalue);
          if (this.game.Data.RegimeObj[this.detailnr].Sleep)
            this.OptionsList3Obj.add("Sleeping", -1, "Yes");
          if (this.game.Data.RegimeObj[this.detailnr].DipBlock)
            this.OptionsList3Obj.add("DipBlock", -1, "Yes");
          if (this.game.Data.RegimeObj[this.detailnr].AI)
          {
            this.OptionsList3Obj.add("AI", -1, "Yes");
            this.OptionsList3Obj.add("Prod Bonus", -1, Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[this.detailnr].ProdBonus)) + "%");
          }
          else if (this.game.Data.RegimeObj[this.detailnr].Version == 0)
            this.OptionsList3Obj.add("Version", -1, "?");
          else
            this.OptionsList3Obj.add("Version", -1, Strings.Trim(Conversion.Str( ( (this.game.Data.RegimeObj[this.detailnr].Version - 314) / 100.0))));
          if (this.OptionsList3Id > 0)
          {
            this.SubPartList[this.SubpartNr(this.OptionsList3Id)].Refresh(this.OptionsList3Obj, tlistselect2);
            this.SubPartFlag[this.SubpartNr(this.OptionsList3Id)] = true;
          }
          else
          {
            tsubpart =  new ATListSubPartClass(this.OptionsList3Obj, 7, 450, tlistselect2, this.game, true, "Regime General", tHighlight: false, tShowPair: true, tValueWidth: 340, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 550, bby: 10);
            this.OptionsList3Id = this.AddSubPart(ref tsubpart, 550, 10, 450, 128, 0);
          }
        }
        if (this.tabsheet == 1)
        {
          this.OptionsList4Obj = ATListClass::new();
          let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
          for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
          {
            if (!this.game.Data.RegimeObj[index].Sleep | !this.game.Data.RegimeObj[index].DipBlock && index != this.detailnr)
            {
              if (this.game.Data.RegimeObj[index].RegimeRel[this.detailnr] == 1)
                tvalue = this.game.Data.RegimeObj[index].RegimeOffer[this.detailnr] != 2 ? (this.game.Data.RegimeObj[this.detailnr].RegimeOffer[index] != 2 ? "Peace" : "Alliance offered by " + this.game.Data.RegimeObj[this.detailnr].Name) : "Alliance offered by " + this.game.Data.RegimeObj[index].Name;
              else if (this.game.Data.RegimeObj[index].RegimeRel[this.detailnr] == 0)
                tvalue = this.game.Data.RegimeObj[index].RegimeOffer[this.detailnr] != 1 ? (this.game.Data.RegimeObj[this.detailnr].RegimeOffer[index] != 1 ? "War" : "Peace offered by " + this.game.Data.RegimeObj[this.detailnr].Name) : "Peace offered by " + this.game.Data.RegimeObj[index].Name;
              else if (this.game.Data.RegimeObj[index].RegimeRel[this.detailnr] == 2)
                tvalue = "Allied".to_owned();
              this.OptionsList4Obj.add(this.game.Data.RegimeObj[index].Name, -1, tvalue);
            }
          }
          if (this.OptionsList4Id > 0)
          {
            this.SubPartList[this.SubpartNr(this.OptionsList4Id)].Refresh(this.OptionsList4Obj, tlistselect2);
            this.SubPartFlag[this.SubpartNr(this.OptionsList4Id)] = true;
          }
          else
          {
            tsubpart =  new ATListSubPartClass(this.OptionsList4Obj, 7, 450, tlistselect2, this.game, true, "Regime Diplomatic", tHighlight: false, tShowPair: true, tValueWidth: 270, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 550, bby: 10);
            this.OptionsList4Id = this.AddSubPart(ref tsubpart, 550, 10, 450, 128, 0);
          }
        }
        if (this.tabsheet == 2)
        {
          this.OptionsList5Obj = ATListClass::new();
          num2 = -1;
          let mut tlistselect3: i32 =  -1;
          if (!this.game.Data.FOWOn | this.game.Data.Turn == this.detailnr)
          {
            if (this.detailnr == this.game.Data.Turn)
            {
              if (this.game.Data.RegimeObj[this.detailnr].TempPPIncrease > 0)
                this.OptionsList5Obj.add("PP", -1, Conversion.Str( this.game.Data.RegimeObj[this.detailnr].ResPts), ".", "+" + Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[this.detailnr].TempPPIncrease)));
              else
                this.OptionsList5Obj.add("PP", -1, Conversion.Str( this.game.Data.RegimeObj[this.detailnr].ResPts), ".", "0");
            }
            else
              this.OptionsList5Obj.add("PP", -1, Conversion.Str( this.game.Data.RegimeObj[this.detailnr].ResPts), ".", ".");
            let mut tdata2: i32 =  0;
            do
            {
              tvalue2: String = ".";
              tvalue3: String = ".";
              let mut Number1: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[tdata2] - this.game.Data.RegimeObj[this.game.Data.Turn].TempRegimeSlotPredict[tdata2];
              if (Number1 > 0)
                tvalue2 = "-" + Strings.Trim(Conversion.Str( Number1));
              let mut Number2: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].TempRegimeSlotIncrease[tdata2];
              if (Number2 > 0)
                tvalue3 = "+" + Strings.Trim(Conversion.Str( Number2));
              if (this.game.Data.Turn != this.detailnr)
              {
                tvalue2 = ".";
                tvalue3 = ".";
              }
              if (this.game.Data.RegimeSlotShow[tdata2] & this.game.Data.RegimeSlotShow2[tdata2] == 0 | this.game.Data.RegimeSlotShow2[tdata2] == 2)
              {
                if (!this.game.Data.FOWOn | this.game.Data.Turn == this.detailnr)
                  this.OptionsList5Obj.add(Strings.Left(this.game.Data.RegimeSlotName[tdata2], 6), tdata2, Conversion.Str( this.game.Data.RegimeObj[this.detailnr].RegimeSlot[tdata2]), tvalue2, tvalue3);
              }
              else if (this.game.Data.RegimeSlotShow2[tdata2] == 1)
                this.OptionsList5Obj.add(Strings.Left(this.game.Data.RegimeSlotName[tdata2], 6), tdata2, Conversion.Str( this.game.Data.RegimeObj[this.detailnr].RegimeSlot[tdata2]), tvalue2, tvalue3);
              else if (this.game.Data.RegimeSlotShow2[tdata2] == 3 & this.game.HandyFunctionsObj.IsAlliedOrSelf(this.detailnr, this.game.Data.Turn))
                this.OptionsList5Obj.add(Strings.Left(this.game.Data.RegimeSlotName[tdata2], 6), tdata2, Conversion.Str( this.game.Data.RegimeObj[this.detailnr].RegimeSlot[tdata2]), tvalue2, tvalue3);
              tdata2 += 1;
            }
            while (tdata2 <= 499);
          }
          else
            this.OptionsList5Obj.add("No info due to FOW", -1);
          if (this.OptionsList5Id > 0)
          {
            this.SubPartList[this.SubpartNr(this.OptionsList5Id)].Refresh(this.OptionsList5Obj, tlistselect3);
            this.SubPartFlag[this.SubpartNr(this.OptionsList5Id)] = true;
          }
          else
          {
            tsubpart =  new ATListSubPartClass(this.OptionsList5Obj, 7, 450, tlistselect3, this.game, true, "Regime Resource", tHighlight: false, tShowPair: true, tValueWidth: 300, tdotopandbottom: false, tbackbitmap: (ref this.OwnBitmap), bbx: 550, bby: 10);
            this.OptionsList5Id = this.AddSubPart(ref tsubpart, 550, 10, 450, 128, 0);
          }
        }
      }
      let mut num3: i32 =  210;
      if (this.detailnr > -1)
      {
        bool flag = true;
        let mut regimeCounter1: i32 =  this.game.Data.RegimeCounter;
        for (let mut index: i32 =  0; index <= regimeCounter1; index += 1)
        {
          if (index != this.game.Data.Turn & index != this.detailnr && this.game.Data.RegimeObj[index].RegimeRel[this.detailnr] == 0 & this.game.Data.RegimeObj[index].RegimeRel[this.game.Data.Turn] == 2)
            flag = false;
          if (index != this.game.Data.Turn & index != this.detailnr && this.game.Data.RegimeObj[index].RegimeRel[this.detailnr] == 2 & this.game.Data.RegimeObj[index].RegimeRel[this.game.Data.Turn] == 0)
            flag = false;
        }
        if ( this.game.Data.RuleVar[524] < 1.0)
          flag = false;
        if (this.detailnr != this.regnr)
        {
          if (this.game.Data.RegimeObj[this.detailnr].DipBlock)
          {
            if (!this.game.Data.RegimeObj[this.detailnr].AI)
            {
              tsubpart =  new TextButtonPartClass("Message", 120, "Send this regime a text message", ref this.OwnBitmap, num3 + 300, 160);
              this.B1Id = this.AddSubPart(ref tsubpart, num3 + 300, 160, 100, 35, 1);
            }
          }
          else
          {
            if (!this.game.Data.RegimeObj[this.detailnr].AI)
            {
              tsubpart =  new TextButtonPartClass("Message", 120, "Send this regime a text message", ref this.OwnBitmap, num3 + 300, 160);
              this.B1Id = this.AddSubPart(ref tsubpart, num3 + 300, 160, 100, 35, 1);
            }
            if (this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime == -1 & this.game.Data.RegimeObj[this.detailnr].UberRegime == -1)
            {
              if (this.game.Data.RegimeObj[this.regnr].RegimeRel[this.detailnr] == 0 & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.regnr] == 0 & this.game.Data.RegimeObj[this.regnr].RegimeOffer[this.detailnr] == 0)
              {
                if (!this.game.Data.RegimeObj[this.detailnr].AI)
                {
                  tsubpart =  new TextButtonPartClass("Offer Peace", 120, "Offer to make peace.", ref this.OwnBitmap, num3 + 0, 160);
                  this.B4Id = this.AddSubPart(ref tsubpart, num3 + 0, 160, 100, 35, 1);
                }
              }
              else if (this.game.Data.RegimeObj[this.regnr].RegimeRel[this.detailnr] == 0 & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.regnr] == 0 & this.game.Data.RegimeObj[this.regnr].RegimeOffer[this.detailnr] == 1)
              {
                tsubpart =  new TextButtonPartClass("Retract Peace", 120, "Retract offer to make peace.", ref this.OwnBitmap, num3 + 0, 160);
                this.B4Id = this.AddSubPart(ref tsubpart, num3 + 0, 160, 100, 35, 1);
              }
              else if (this.game.Data.RegimeObj[this.regnr].RegimeRel[this.detailnr] == 0 & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.regnr] == 1)
              {
                tsubpart =  new TextButtonPartClass("Accept Peace", 120, "Accept the peace offer of the other regime.", ref this.OwnBitmap, num3 + 0, 160);
                this.B4Id = this.AddSubPart(ref tsubpart, num3 + 0, 160, 100, 35, 1);
              }
              else if (this.game.Data.RegimeObj[this.regnr].RegimeRel[this.detailnr] == 1)
              {
                if ( this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >=  this.game.Data.RuleVar[818])
                {
                  tsubpart =  new TextButtonPartClass("Declare War", 120, "Declare war on selected regime", ref this.OwnBitmap, num3 + 0, 160);
                  this.B2Id = this.AddSubPart(ref tsubpart, num3 + 0, 160, 100, 35, 1);
                }
                else
                {
                  tsubpart =  new TextButtonPartClass("Declare War", 120, "You do not have the required " + this.game.Data.RuleVar[818].ToString() + " points to declare war.", ref this.OwnBitmap, num3 + 0, 160, true);
                  this.B2Id = this.AddSubPart(ref tsubpart, num3 + 0, 160, 100, 35, 0);
                }
                if (flag & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.regnr] == 0 & this.game.Data.RegimeObj[this.regnr].RegimeOffer[this.detailnr] == 0)
                {
                  if (!this.game.Data.RegimeObj[this.detailnr].AI)
                  {
                    tsubpart =  new TextButtonPartClass("Offer Alliance", 120, "Offer an alliance.", ref this.OwnBitmap, num3 + 150, 160);
                    this.B5Id = this.AddSubPart(ref tsubpart, num3 + 150, 160, 120, 35, 1);
                  }
                }
                else if (this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.regnr] == 0 & this.game.Data.RegimeObj[this.regnr].RegimeOffer[this.detailnr] == 2 & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.regnr] == 0)
                {
                  tsubpart =  new TextButtonPartClass("Retract Alliance", 120, "Retract offer for an alliance.", ref this.OwnBitmap, num3 + 150, 160);
                  this.B5Id = this.AddSubPart(ref tsubpart, num3 + 150, 160, 120, 35, 1);
                }
                else if (flag & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.regnr] == 2)
                {
                  tsubpart =  new TextButtonPartClass("Accept Alliance", 120, "Accept alliance offer.", ref this.OwnBitmap, num3 + 150, 160);
                  this.B5Id = this.AddSubPart(ref tsubpart, num3 + 150, 160, 120, 35, 1);
                }
              }
              else if (this.game.Data.RegimeObj[this.regnr].RegimeRel[this.detailnr] == 2)
              {
                if ( this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >=  this.game.Data.RuleVar[818])
                {
                  tsubpart =  new TextButtonPartClass("Declare War", 120, "Declare war on selected regime", ref this.OwnBitmap, num3 + 0, 160);
                  this.B2Id = this.AddSubPart(ref tsubpart, num3 + 0, 160, 100, 35, 1);
                }
                else
                {
                  tsubpart =  new TextButtonPartClass("Declare War", 120, "You do not have the required " + this.game.Data.RuleVar[818].ToString() + " points to declare war.", ref this.OwnBitmap, num3 + 0, 160, true);
                  this.B2Id = this.AddSubPart(ref tsubpart, num3 + 0, 160, 100, 35, 0);
                }
              }
            }
          }
        }
        if (this.game.Data.PasswordsOn && this.detailnr != this.game.Data.Turn & !this.game.Data.RegimeObj[this.detailnr].AI & this.game.HandyFunctionsObj.GetHumanPlayers() > 2)
        {
          let mut num4: i32 =  0;
          let mut num5: i32 =  0;
          let mut num6: i32 =  this.game.Data.Turn + 1;
          let mut regimeCounter2: i32 =  this.game.Data.RegimeCounter;
          for (let mut index: i32 =  num6; index <= regimeCounter2; index += 1)
          {
            if (index == this.detailnr & num4 == 0)
              num5 = 1;
            if (!this.game.Data.RegimeObj[index].AI & num5 == 0)
              num4 = 1;
          }
          if (num4 == -1)
          {
            let mut num7: i32 =  this.game.Data.Turn - 1;
            for (let mut index: i32 =  0; index <= num7; index += 1)
            {
              if (index == this.detailnr & num4 == 0)
                num5 = 1;
              if (!this.game.Data.RegimeObj[index].AI & num5 == 0)
                num4 = 1;
            }
          }
          if (num5 == 0)
          {
            tsubpart =  new TextButtonPartClass("Pass Change", 120, "Change password off inactive player so somebody else can take over", ref this.OwnBitmap, num3 + 450, 160);
            this.B3Id = this.AddSubPart(ref tsubpart, num3 + 450, 160, 100, 35, 1);
          }
        }
      }
      if (this.B1Id == 0)
      {
        tsubpart =  new TextButtonPartClass("Message", 120, "You cannot send text message to AI regimes.", ref this.OwnBitmap, num3 + 300, 160, true);
        this.F1id = this.AddSubPart(ref tsubpart, num3 + 300, 160, 100, 35, 1);
      }
      if (this.B2Id == 0 & this.B4Id == 0)
      {
        tsubpart =  new TextButtonPartClass("Declare War", 120, "You cannot declare war on this regime.", ref this.OwnBitmap, num3 + 0, 160, true);
        this.F2id = this.AddSubPart(ref tsubpart, num3 + 0, 160, 100, 35, 1);
      }
      if (this.B3Id == 0)
      {
        tsubpart =  new TextButtonPartClass("Pass Change", 120, "You can only change the password of a human player in a 3+ human player game; and not the next human player.", ref this.OwnBitmap, num3 + 450, 160, true);
        this.F3id = this.AddSubPart(ref tsubpart, num3 + 450, 160, 100, 35, 1);
      }
      if (this.B5Id == 0)
      {
        tsubpart =  new TextButtonPartClass("Offer Alliance", 120, "You cannot offer alliance to this regime..", ref this.OwnBitmap, num3 + 150, 160, true);
        this.F5id = this.AddSubPart(ref tsubpart, num3 + 150, 160, 120, 35, 1);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.B2Id)
            {
              if ( this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >=  this.game.Data.RuleVar[818])
              {
                if ( this.game.Data.RuleVar[818] < 1.0 | Interaction.MsgBox( ("Declaring war wil cost " + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[818])) + " PP. Are you sure"), MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                {
                  OrderResult orderResult = this.game.ProcessingObj.DeclareWar(this.regnr, this.detailnr);
                  let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                  for (let mut onregnr: i32 =  0; onregnr <= regimeCounter; onregnr += 1)
                  {
                    if (onregnr != this.game.Data.Turn & onregnr != this.detailnr && this.game.Data.RegimeObj[onregnr].RegimeRel[this.detailnr] == 2)
                    {
                      let mut num2: i32 =   Interaction.MsgBox( ("Also declared war on " + this.game.Data.RegimeObj[onregnr].Name), Title: ( "Shadow Empire : Planetary Conquest"));
                      orderResult = this.game.ProcessingObj.DeclareWar(this.regnr, onregnr);
                      if ( this.game.Data.RuleVar[818] > 0.0)
                      {
                        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
                        RegimeClass[] regimeClassArray = regimeObj;
                        let mut turn: i32 =  this.game.Data.Turn;
                        let mut index2: i32 =  turn;
                        regimeClassArray[index2].ResPts =  Math.Round( ( regimeObj[turn].ResPts + this.game.Data.RuleVar[818]));
                      }
                    }
                  }
                  let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                  for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
                  {
                    if (this.game.Data.UnitObj[index3].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index3].PreDef == -1)
                      this.game.Data.UnitObj[index3].LastAP = -1;
                  }
                  if (Strings.Len(orderResult.ErrorString) > 0)
                  {
                    let mut num3: i32 =   Interaction.MsgBox( (orderResult.ErrorString + " battles have been fought due to your declaration of war."), Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                }
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == this.B1Id)
              {
                Form2::new( this.formref).Initialize(this.game.Data, 7, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B3Id)
              {
                if (MsgBoxResult.Yes == Interaction.MsgBox( "Only change the password of a regime that has become inactive so that you give it a new player. Are you sure you want to change the password of this regime?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest"))
                {
                  this.game.Data.RegimeObj[this.detailnr].PassWord = Interaction.InputBox("Give new password", "Shadow Empire : Planetary Conquest");
                  let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
                  for (let mut index4: i32 =  0; index4 <= regimeCounter; index4 += 1)
                  {
                    RegimeClass[] regimeObj = this.game.Data.RegimeObj;
                    RegimeClass[] regimeClassArray = regimeObj;
                    let mut index5: i32 =  index4;
                    let mut index6: i32 =  index5;
                    regimeClassArray[index6].MessCounter = regimeObj[index5].MessCounter + 1;
                    let mut messCounter: i32 =  this.game.Data.RegimeObj[index4].MessCounter;
                    this.game.Data.RegimeObj[index4].MessString = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index4].MessString, (Array) new string[messCounter + 1]);
                    this.game.Data.RegimeObj[index4].MessBackPic = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index4].MessBackPic, (Array) new int[messCounter + 1]);
                    this.game.Data.RegimeObj[index4].MessFrontPic = (int[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index4].MessFrontPic, (Array) new int[messCounter + 1]);
                    this.game.Data.RegimeObj[index4].MessWav = (string[]) Utils.CopyArray((Array) this.game.Data.RegimeObj[index4].MessWav, (Array) new string[messCounter + 1]);
                    this.game.Data.RegimeObj[index4].MessString[messCounter] = "Password Changed!\r\n\r\n" + this.game.Data.RegimeObj[this.game.Data.Turn].Name + " has changed the password of " + this.game.Data.RegimeObj[this.detailnr].Name + ".";
                    this.game.Data.RegimeObj[index4].MessBackPic[messCounter] = -2;
                    this.game.Data.RegimeObj[index4].MessFrontPic[messCounter] = -1;
                  }
                  let mut num4: i32 =   Interaction.MsgBox( "Done! Password has been reset. ", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              OrderResult orderResult;
              if (num1 == this.B4Id)
              {
                if (this.game.Data.RegimeObj[this.game.Data.Turn].RegimeOffer[this.detailnr] == 0 & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.game.Data.Turn] == 1)
                  orderResult = this.game.ProcessingObj.MakePeace(this.regnr, this.detailnr);
                else if (this.game.Data.RegimeObj[this.game.Data.Turn].RegimeOffer[this.detailnr] == 0 & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.game.Data.Turn] == 0)
                {
                  this.game.Data.RegimeObj[this.game.Data.Turn].RegimeOffer[this.detailnr] = 1;
                  this.game.HandyFunctionsObj.AddMessageForAll(this.game.Data.RegimeObj[this.game.Data.Turn].Name + " offers peace to " + this.game.Data.RegimeObj[this.detailnr].Name, -1, 1);
                }
                else if (this.game.Data.RegimeObj[this.game.Data.Turn].RegimeOffer[this.detailnr] == 1 & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.game.Data.Turn] == 0)
                {
                  this.game.Data.RegimeObj[this.game.Data.Turn].RegimeOffer[this.detailnr] = 0;
                  this.game.HandyFunctionsObj.AddMessageForAll(this.game.Data.RegimeObj[this.game.Data.Turn].Name + " retracts peace offer to " + this.game.Data.RegimeObj[this.detailnr].Name, -1, 1);
                }
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.B5Id)
              {
                if (this.game.Data.RegimeObj[this.game.Data.Turn].RegimeOffer[this.detailnr] == 0 & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.game.Data.Turn] == 2)
                  orderResult = this.game.ProcessingObj.MakeAlliance(this.regnr, this.detailnr);
                else if (this.game.Data.RegimeObj[this.game.Data.Turn].RegimeOffer[this.detailnr] == 0 & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.game.Data.Turn] == 0)
                {
                  this.game.Data.RegimeObj[this.game.Data.Turn].RegimeOffer[this.detailnr] = 2;
                  this.game.HandyFunctionsObj.AddMessageForAll(this.game.Data.RegimeObj[this.game.Data.Turn].Name + " offers alliance to " + this.game.Data.RegimeObj[this.detailnr].Name, -1, 1);
                }
                else if (this.game.Data.RegimeObj[this.game.Data.Turn].RegimeOffer[this.detailnr] == 2 & this.game.Data.RegimeObj[this.detailnr].RegimeOffer[this.game.Data.Turn] == 0)
                {
                  this.game.HandyFunctionsObj.AddMessageForAll(this.game.Data.RegimeObj[this.game.Data.Turn].Name + " retracts alliance offer to " + this.game.Data.RegimeObj[this.detailnr].Name, -1, 1);
                  this.game.Data.RegimeObj[this.game.Data.Turn].RegimeOffer[this.detailnr] = 0;
                }
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.Tab1id)
              {
                if (this.OptionsList3Id > 0)
                {
                  this.RemoveSubPart(this.OptionsList3Id);
                  this.OptionsList3Id = 0;
                }
                if (this.OptionsList4Id > 0)
                {
                  this.RemoveSubPart(this.OptionsList4Id);
                  this.OptionsList4Id = 0;
                }
                if (this.OptionsList5Id > 0)
                {
                  this.RemoveSubPart(this.OptionsList5Id);
                  this.OptionsList5Id = 0;
                }
                this.tabsheet = 0;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.tab2id)
              {
                if (this.OptionsList3Id > 0)
                {
                  this.RemoveSubPart(this.OptionsList3Id);
                  this.OptionsList3Id = 0;
                }
                if (this.OptionsList4Id > 0)
                {
                  this.RemoveSubPart(this.OptionsList4Id);
                  this.OptionsList4Id = 0;
                }
                if (this.OptionsList5Id > 0)
                {
                  this.RemoveSubPart(this.OptionsList5Id);
                  this.OptionsList5Id = 0;
                }
                this.tabsheet = 1;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.tab3id)
              {
                if (this.OptionsList3Id > 0)
                {
                  this.RemoveSubPart(this.OptionsList3Id);
                  this.OptionsList3Id = 0;
                }
                if (this.OptionsList4Id > 0)
                {
                  this.RemoveSubPart(this.OptionsList4Id);
                  this.OptionsList4Id = 0;
                }
                if (this.OptionsList5Id > 0)
                {
                  this.RemoveSubPart(this.OptionsList5Id);
                  this.OptionsList5Id = 0;
                }
                this.tabsheet = 2;
                this.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.OptionsListId)
              {
                let mut num5: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num5 > -1)
                {
                  this.detailnr = num5;
                  this.dostuff();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              num6: i32;
              if (num1 == this.OptionsList2Id)
              {
                num6 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.OptionsList3Id)
              {
                num6 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.OptionsList4Id)
              {
                num6 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.OptionsList5Id)
              {
                num6 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
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
