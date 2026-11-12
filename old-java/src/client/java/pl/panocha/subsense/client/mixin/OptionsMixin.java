package pl.panocha.subsense.client.mixin;

import com.mojang.serialization.Codec;
import net.minecraft.client.OptionInstance;
import net.minecraft.client.Options;
import net.minecraft.network.chat.Component;

import org.jspecify.annotations.NullMarked;
import org.spongepowered.asm.mixin.Final;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Mutable;
import org.spongepowered.asm.mixin.Shadow;
import org.spongepowered.asm.mixin.Unique;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

import java.util.Optional;

@NullMarked
@Mixin(Options.class)
public abstract class OptionsMixin {

    @Unique
    private static final double SUBSENSE_MIN = -1.0 / 3.0;

    @Unique
    private static final double SUBSENSE_MAX = 1.0;

    @Unique
    private static final Codec<Double> SUBSENSE_CODEC =
            Codec.doubleRange(SUBSENSE_MIN, SUBSENSE_MAX);

    @Unique
    private static final OptionInstance.SliderableValueSet<Double> SUBSENSE_RANGE =
            new OptionInstance.SliderableValueSet<>() {

                private Double exactValue;
                private long exactSliderBits;
                private boolean hasExactValue;

                @Override
                public Optional<Double> validateValue(Double value) {
                    if (!Double.isFinite(value)) {
                        return Optional.empty();
                    }

                    if (value < SUBSENSE_MIN || value > SUBSENSE_MAX) {
                        return Optional.empty();
                    }

                    return Optional.of(value);
                }

                @Override
                public double toSliderValue(Double value) {
                    double slider = Math.clamp(
                            (value - SUBSENSE_MIN)
                                    / (SUBSENSE_MAX - SUBSENSE_MIN),
                            0.0,
                            1.0
                    );

                    this.exactValue = value;
                    this.exactSliderBits = Double.doubleToRawLongBits(slider);
                    this.hasExactValue = true;

                    return slider;
                }

                @Override
                public Double fromSliderValue(double slider) {
                    slider = Math.clamp(slider, 0.0, 1.0);

                    if (this.hasExactValue
                            && Double.doubleToRawLongBits(slider) == this.exactSliderBits) {
                        return this.exactValue;
                    }

                    return SUBSENSE_MIN
                            + slider * (SUBSENSE_MAX - SUBSENSE_MIN);
                }

                @Override
                public Codec<Double> codec() {
                    return SUBSENSE_CODEC;
                }
            };

    @Shadow
    @Final
    @Mutable
    private OptionInstance<Double> sensitivity;

    @Inject(
            method = "<init>",
            at = @At(
                    value = "INVOKE",
                    target = "Lnet/minecraft/client/Options;load()V"
            )
    )
    private void subsense$replaceSensitivityOption(CallbackInfo ci) {
        this.sensitivity = new OptionInstance<>(
                "options.sensitivity",
                OptionInstance.noTooltip(),
                OptionsMixin::subsense$formatSensitivity,
                SUBSENSE_RANGE,
                SUBSENSE_CODEC,
                0.5,
                value -> {
                }
        );
    }

    @Unique
    private static Component subsense$formatSensitivity(
            Component caption,
            Double value
    ) {
        if (value >= 1.0) {
            return Options.genericValueLabel(
                    caption,
                    Component.translatable("options.sensitivity.max")
            );
        }

        long percentage = Math.round(value * 200.0);

        return Component.translatable(
                "options.percent_value",
                caption,
                percentage
        );
    }
}